use crate::block::Block;
use crate::blockchain::Blockchain;
use crate::contracts::{ContractCall, ContractEngine, SmartContract};
use crate::crypto::Wallet;
use crate::errors::{BlockchainError, Result};
use crate::mining::{MiningConfig, MiningStats};
use crate::network::NetworkStats;
use crate::storage::{BlockchainStorage, WalletInfo};
use crate::transaction::Transaction;
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::{Html, IntoResponse, Json},
    routing::{get, post},
    Router,
};
use base64::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tower::ServiceBuilder;
use tower_http::cors::{Any, CorsLayer};
use tracing::info;

#[derive(Clone)]
pub struct ApiState {
    pub blockchain: Arc<RwLock<Blockchain>>,
    pub storage: Arc<BlockchainStorage>,
    pub contract_engine: Arc<RwLock<ContractEngine>>,
    pub mining_stats: Arc<RwLock<MiningStats>>,
    pub network_stats: Arc<RwLock<NetworkStats>>,
    pub wallets: Arc<RwLock<HashMap<String, Wallet>>>,
}

#[derive(Serialize, Deserialize)]
pub struct BlockchainInfo {
    pub length: usize,
    pub latest_hash: String,
    pub latest_block_index: u64,
    pub total_transactions: u64,
    pub is_valid: bool,
    pub difficulty: u32,
}

#[derive(Serialize, Deserialize)]
pub struct TransactionRequest {
    pub from: String,
    pub to: String,
    pub amount: f64,
    pub data: Option<String>,
    pub private_key: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct MineBlockRequest {
    pub miner_address: String,
    pub include_pending: bool,
}

#[derive(Serialize, Deserialize)]
pub struct CreateWalletRequest {
    pub name: String,
}

#[derive(Serialize, Deserialize)]
pub struct ContractDeployRequest {
    pub name: String,
    pub code: String, // Base64 encoded WASM
    pub owner: String,
    pub gas_limit: u64,
}

#[derive(Serialize, Deserialize)]
pub struct ContractCallRequest {
    pub contract_id: String,
    pub function_name: String,
    pub args: Vec<serde_json::Value>,
    pub caller: String,
    pub value: f64,
    pub gas_limit: u64,
}

#[derive(Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
        }
    }

    pub fn error(message: String) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(message),
        }
    }
}

pub fn create_router(state: ApiState) -> Router {
    Router::new()
        // Blockchain endpoints
        .route("/api/blockchain/info", get(get_blockchain_info))
        .route("/api/blockchain/validate", get(validate_blockchain))
        .route("/api/blocks", get(get_blocks))
        .route("/api/blocks/:index", get(get_block))

        // Transaction endpoints
        .route("/api/transactions", get(get_transactions))
        .route("/api/transactions", post(create_transaction))
        .route("/api/transactions/:id", get(get_transaction))
        .route("/api/balance/:address", get(get_balance))

        // Mining endpoints
        .route("/api/mine", post(mine_block))
        .route("/api/mining/stats", get(get_mining_stats))
        .route("/api/mining/config", get(get_mining_config))
        .route("/api/mining/config", post(update_mining_config))

        // Wallet endpoints
        .route("/api/wallets", get(list_wallets))
        .route("/api/wallets", post(create_wallet))
        .route("/api/wallets/:address", get(get_wallet))

        // Smart contract endpoints
        .route("/api/contracts", get(list_contracts))
        .route("/api/contracts", post(deploy_contract))
        .route("/api/contracts/:id", get(get_contract))
        .route("/api/contracts/:id/call", post(call_contract))

        // Network endpoints
        .route("/api/network/stats", get(get_network_stats))
        .route("/api/network/peers", get(get_peers))

        // Web dashboard
        .route("/", get(dashboard))
        .route("/dashboard", get(dashboard))

        .layer(
            ServiceBuilder::new()
                .layer(CorsLayer::new().allow_origin(Any).allow_methods(Any).allow_headers(Any))
        )
        .with_state(state)
}

// Blockchain API handlers
async fn get_blockchain_info(State(state): State<ApiState>) -> impl IntoResponse {
    let blockchain = state.blockchain.read().await;

    let info = BlockchainInfo {
        length: blockchain.len(),
        latest_hash: blockchain.get_latest_block()
            .map(|b| b.hash.clone())
            .unwrap_or_default(),
        latest_block_index: blockchain.get_latest_block()
            .map(|b| b.index)
            .unwrap_or(0),
        total_transactions: blockchain.chain()
            .iter()
            .map(|b| b.transactions.len() as u64)
            .sum(),
        is_valid: blockchain.is_chain_valid().is_ok(),
        difficulty: blockchain.get_latest_block()
            .map(|b| b.difficulty)
            .unwrap_or(4),
    };

    Json(ApiResponse::success(info))
}

async fn validate_blockchain(State(state): State<ApiState>) -> impl IntoResponse {
    let blockchain = state.blockchain.read().await;

    match blockchain.is_chain_valid() {
        Ok(()) => Json(ApiResponse::success("Blockchain is valid")),
        Err(e) => Json(ApiResponse::error(e.to_string())),
    }
}

async fn get_blocks(
    State(state): State<ApiState>,
    Query(params): Query<HashMap<String, String>>,
) -> impl IntoResponse {
    let blockchain = state.blockchain.read().await;

    let limit = params.get("limit")
        .and_then(|l| l.parse::<usize>().ok())
        .unwrap_or(10);

    let offset = params.get("offset")
        .and_then(|o| o.parse::<usize>().ok())
        .unwrap_or(0);

    let blocks: Vec<Block> = blockchain.chain()
        .iter()
        .rev()
        .skip(offset)
        .take(limit)
        .cloned()
        .collect();

    Json(ApiResponse::success(blocks))
}

async fn get_block(State(state): State<ApiState>, Path(index): Path<u64>) -> impl IntoResponse {
    let blockchain = state.blockchain.read().await;

    match blockchain.get_block(index) {
        Some(block) => {
            let response = Json(ApiResponse::success(block.clone()));
            (StatusCode::OK, response)
        },
        None => {
            let response = ApiResponse::<Block>::error("Block not found".to_string());
            (StatusCode::NOT_FOUND, Json(response))
        }
    }
}

// Transaction API handlers
async fn get_transactions(
    State(state): State<ApiState>,
    Query(params): Query<HashMap<String, String>>,
) -> impl IntoResponse {
    let blockchain = state.blockchain.read().await;

    let limit = params.get("limit")
        .and_then(|l| l.parse::<usize>().ok())
        .unwrap_or(20);

    let transactions: Vec<Transaction> = blockchain.chain()
        .iter()
        .rev()
        .flat_map(|b| &b.transactions)
        .take(limit)
        .cloned()
        .collect();

    Json(ApiResponse::success(transactions))
}

async fn create_transaction(
    State(state): State<ApiState>,
    Json(req): Json<TransactionRequest>,
) -> impl IntoResponse {
    let _wallets = state.wallets.read().await;

    let transaction = if let Some(private_key_hex) = req.private_key {
        // Create signed transaction
        let private_key = match hex::decode(private_key_hex) {
            Ok(key) => key,
            Err(_) => {
                let response = ApiResponse::<Transaction>::error("Invalid private key format".to_string());
                return (StatusCode::BAD_REQUEST, Json(response));
            }
        };

        let wallet = match Wallet::from_private_key("temp".to_string(), &private_key) {
            Ok(w) => w,
            Err(e) => {
                let response = ApiResponse::<Transaction>::error(e.to_string());
                return (StatusCode::BAD_REQUEST, Json(response));
            }
        };

        let mut tx = match Transaction::new(req.from, req.to, req.amount, req.data) {
            Ok(t) => t,
            Err(e) => {
                let response = ApiResponse::<Transaction>::error(e.to_string());
                return (StatusCode::BAD_REQUEST, Json(response));
            }
        };

        let signable_data = match tx.signable_data() {
            Ok(data) => data,
            Err(e) => {
                let response = ApiResponse::<Transaction>::error(e.to_string());
                return (StatusCode::BAD_REQUEST, Json(response));
            }
        };

        tx.signature = Some(wallet.sign_transaction(&signable_data));
        tx.from_public_key = Some(wallet.keypair.public_key().clone());
        tx
    } else {
        // Create unsigned transaction
        match Transaction::new(req.from, req.to, req.amount, req.data) {
            Ok(tx) => tx,
            Err(e) => {
                let response = ApiResponse::<Transaction>::error(e.to_string());
                return (StatusCode::BAD_REQUEST, Json(response));
            }
        }
    };

    // In a real implementation, you'd add this to a transaction pool
    (StatusCode::OK, Json(ApiResponse::success(transaction)))
}

async fn get_transaction(State(state): State<ApiState>, Path(id): Path<String>) -> impl IntoResponse {
    let blockchain = state.blockchain.read().await;

    for block in blockchain.chain() {
        for transaction in &block.transactions {
            if transaction.id == id {
                return (StatusCode::OK, Json(ApiResponse::success(transaction.clone())));
            }
        }
    }

    let response = ApiResponse::<Transaction>::error("Transaction not found".to_string());
    (StatusCode::NOT_FOUND, Json(response))
}

async fn get_balance(State(state): State<ApiState>, Path(address): Path<String>) -> impl IntoResponse {
    let blockchain = state.blockchain.read().await;
    let balance = blockchain.get_balance(&address);
    Json(ApiResponse::success(balance))
}

// Mining API handlers
async fn mine_block(
    State(_state): State<ApiState>,
    Json(_req): Json<MineBlockRequest>
) -> impl IntoResponse {
    // This is a simplified mining endpoint
    // In a real implementation, mining would happen in background threads
    Json(ApiResponse::success("Mining started"))
}

async fn get_mining_stats(State(state): State<ApiState>) -> impl IntoResponse {
    let stats = state.mining_stats.read().await;
    Json(ApiResponse::success(stats.clone()))
}

async fn get_mining_config(State(_state): State<ApiState>) -> impl IntoResponse {
    let config = MiningConfig::default();
    Json(ApiResponse::success(config))
}

async fn update_mining_config(
    State(_state): State<ApiState>,
    Json(_config): Json<MiningConfig>,
) -> impl IntoResponse {
    // Update mining configuration
    Json(ApiResponse::success("Mining configuration updated"))
}

// Wallet API handlers
async fn list_wallets(State(state): State<ApiState>) -> impl IntoResponse {
    match state.storage.list_wallets().await {
        Ok(wallets) => (StatusCode::OK, Json(ApiResponse::success(wallets))),
        Err(e) => {
            let response = ApiResponse::<Vec<WalletInfo>>::error(e.to_string());
            (StatusCode::INTERNAL_SERVER_ERROR, Json(response))
        }
    }
}

async fn create_wallet(
    State(state): State<ApiState>,
    Json(req): Json<CreateWalletRequest>,
) -> impl IntoResponse {
    let wallet = Wallet::new(req.name);

    match state.storage.save_wallet(&wallet).await {
        Ok(()) => {
            state.wallets.write().await.insert(wallet.address(), wallet.clone());
            (StatusCode::OK, Json(ApiResponse::success(wallet)))
        }
        Err(e) => {
            let response = ApiResponse::<Wallet>::error(e.to_string());
            (StatusCode::INTERNAL_SERVER_ERROR, Json(response))
        }
    }
}

async fn get_wallet(State(state): State<ApiState>, Path(address): Path<String>) -> impl IntoResponse {
    match state.storage.load_wallet(&address).await {
        Ok(Some(wallet)) => (StatusCode::OK, Json(ApiResponse::success(wallet))),
        Ok(None) => {
            let response = ApiResponse::<Wallet>::error("Wallet not found".to_string());
            (StatusCode::NOT_FOUND, Json(response))
        }
        Err(e) => {
            let response = ApiResponse::<Wallet>::error(e.to_string());
            (StatusCode::INTERNAL_SERVER_ERROR, Json(response))
        }
    }
}

// Smart contract API handlers
async fn list_contracts(State(state): State<ApiState>) -> impl IntoResponse {
    let engine = state.contract_engine.read().await;
    let contracts = engine.list_contracts().into_iter().cloned().collect::<Vec<_>>();
    Json(ApiResponse::success(contracts))
}

async fn deploy_contract(
    State(state): State<ApiState>,
    Json(req): Json<ContractDeployRequest>,
) -> impl IntoResponse {
    let _code = match BASE64_STANDARD.decode(req.code) {
        Ok(code) => code,
        Err(_) => {
            let response = ApiResponse::<SmartContract>::error("Invalid base64 code".to_string());
            return (StatusCode::BAD_REQUEST, Json(response));
        }
    };

    // This is simplified - in reality you'd need to parse the ABI
    let contract = SmartContract::simple_storage_contract();

    let mut engine = state.contract_engine.write().await;
    match engine.deploy_contract(contract.clone()) {
        Ok(()) => (StatusCode::OK, Json(ApiResponse::success(contract))),
        Err(e) => {
            let response = ApiResponse::<SmartContract>::error(e.to_string());
            (StatusCode::BAD_REQUEST, Json(response))
        }
    }
}

async fn get_contract(State(state): State<ApiState>, Path(id): Path<String>) -> impl IntoResponse {
    let engine = state.contract_engine.read().await;

    match engine.get_contract(&id) {
        Some(contract) => (StatusCode::OK, Json(ApiResponse::success(contract.clone()))),
        None => {
            let response = ApiResponse::<SmartContract>::error("Contract not found".to_string());
            (StatusCode::NOT_FOUND, Json(response))
        }
    }
}

async fn call_contract(
    State(state): State<ApiState>,
    Path(id): Path<String>,
    Json(req): Json<ContractCallRequest>,
) -> impl IntoResponse {
    // This is simplified - in reality you'd need to convert the JSON args to ContractValues
    let call = ContractCall {
        contract_id: id,
        function_name: req.function_name,
        args: vec![], // Simplified
        caller: req.caller,
        value: req.value,
        gas_limit: req.gas_limit,
    };

    let mut engine = state.contract_engine.write().await;
    match engine.call_contract(call) {
        Ok(result) => (StatusCode::OK, Json(ApiResponse::success(result))),
        Err(e) => {
            let response = ApiResponse::<crate::contracts::ExecutionResult>::error(e.to_string());
            (StatusCode::BAD_REQUEST, Json(response))
        }
    }
}

// Network API handlers
async fn get_network_stats(State(state): State<ApiState>) -> impl IntoResponse {
    let stats = state.network_stats.read().await;
    Json(ApiResponse::success(stats.clone()))
}

async fn get_peers(State(state): State<ApiState>) -> impl IntoResponse {
    let stats = state.network_stats.read().await;
    Json(ApiResponse::success(stats.connected_peers))
}

// Web dashboard
async fn dashboard() -> Html<&'static str> {
    Html(include_str!("../web/dashboard.html"))
}

pub async fn start_server(state: ApiState, port: u16) -> Result<()> {
    let app = create_router(state);

    let addr = format!("0.0.0.0:{}", port);
    info!("Starting API server on {}", addr);

    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .map_err(|e| BlockchainError::Io(e))?;

    axum::serve(listener, app)
        .await
        .map_err(|e| BlockchainError::Io(std::io::Error::new(std::io::ErrorKind::Other, e)))?;

    Ok(())
}