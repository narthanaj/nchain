use anyhow::Result;
use clap::{Parser, Subcommand};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, Level};
use tracing_subscriber;

use blockchain::{
    api::{start_server, ApiState},
    cli::*,
    contracts::ContractEngine,
    crypto::Wallet,
    network::{NetworkConfig, NetworkStats, P2PNode},
    storage::BlockchainStorage,
    Blockchain,
};

#[derive(Parser)]
#[command(name = "blockchain")]
#[command(about = "Advanced Rust blockchain with smart contracts, P2P networking, and web interface")]
#[command(version = "2.0.0")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "Start interactive CLI mode")]
    Interactive,

    #[command(about = "Start full node with API server")]
    Node {
        #[arg(long, default_value = "8080")]
        api_port: u16,
        #[arg(long, default_value = "9000")]
        p2p_port: u16,
        #[arg(long, default_value = "blockchain.db")]
        database: String,
    },

    #[command(about = "Start API server only")]
    Api {
        #[arg(long, default_value = "8080")]
        port: u16,
        #[arg(long, default_value = "blockchain.db")]
        database: String,
    },

    #[command(about = "Create a new wallet")]
    CreateWallet {
        #[arg(help = "Wallet name")]
        name: String,
    },

    #[command(about = "Mine a block")]
    Mine {
        #[arg(help = "Miner wallet address")]
        miner_address: String,
        #[arg(long, default_value = "4")]
        difficulty: u32,
    },

    #[command(about = "Add a new transaction")]
    Transaction {
        #[arg(help = "From address")]
        from: String,
        #[arg(help = "To address")]
        to: String,
        #[arg(help = "Amount")]
        amount: f64,
        #[arg(help = "Optional data")]
        data: Option<String>,
    },

    #[command(about = "Show blockchain information")]
    Info,

    #[command(about = "Validate the blockchain")]
    Validate,

    #[command(about = "Get balance for an address")]
    Balance {
        #[arg(help = "Address to check")]
        address: String,
    },

    #[command(about = "List all wallets")]
    ListWallets,

    #[command(about = "Deploy a smart contract")]
    DeployContract {
        #[arg(help = "Contract name")]
        name: String,
        #[arg(help = "WASM file path")]
        wasm_file: String,
        #[arg(help = "Owner address")]
        owner: String,
    },

    #[command(about = "Call a smart contract")]
    CallContract {
        #[arg(help = "Contract ID")]
        contract_id: String,
        #[arg(help = "Function name")]
        function: String,
        #[arg(help = "Caller address")]
        caller: String,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .init();

    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Interactive) | None => {
            info!("Starting interactive mode");
            let mut interactive = InteractiveMode::new()?;
            interactive.run()?;
        }

        Some(Commands::Node { api_port, p2p_port, database }) => {
            info!("Starting full node with API server on port {} and P2P on port {}", api_port, p2p_port);
            start_full_node(api_port, p2p_port, &database).await?;
        }

        Some(Commands::Api { port, database }) => {
            info!("Starting API server on port {}", port);
            start_api_server(port, &database).await?;
        }

        Some(Commands::CreateWallet { name }) => {
            let storage = BlockchainStorage::create_file(&format!("{}.db", "blockchain")).await?;
            let wallet = Wallet::new(name.clone());

            storage.save_wallet(&wallet).await?;

            println!("✅ Wallet created successfully!");
            println!("Name: {}", wallet.name);
            println!("Address: {}", wallet.address());
            println!("Public Key: {}", wallet.keypair.public_key());
        }

        Some(Commands::Mine { miner_address: _, difficulty: _ }) => {
            println!("⛏️ Mining is not implemented in CLI mode. Use the full node or API.");
        }

        Some(Commands::Transaction { from, to, amount, data }) => {
            let storage = BlockchainStorage::create_file("blockchain.db").await?;
            let _blockchain = Blockchain::new()?;

            // Load existing blocks
            let blocks = storage.load_all_blocks().await?;
            for block in blocks {
                // In a real implementation, you'd need to properly reconstruct the blockchain
                println!("Loaded block #{}", block.index);
            }

            let transaction = blockchain::Transaction::new(from, to, amount, data)?;

            println!("✅ Transaction created: {}", transaction.id);
            println!("💡 Add this transaction to a block using the mining feature");
        }

        Some(Commands::Info) => {
            let blockchain = Blockchain::new()?;
            println!("🔗 Blockchain Information:");
            println!("  Length: {} blocks", blockchain.len());
            println!("  Valid: {}", blockchain.is_chain_valid().is_ok());
            if let Ok(latest) = blockchain.get_latest_block() {
                println!("  Latest block: #{}", latest.index);
                println!("  Latest hash: {}", &latest.hash[..16]);
            }
        }

        Some(Commands::Validate) => {
            let blockchain = Blockchain::new()?;
            match blockchain.is_chain_valid() {
                Ok(()) => println!("✅ Blockchain is valid!"),
                Err(e) => {
                    println!("❌ Blockchain validation failed: {}", e);
                    std::process::exit(1);
                }
            }
        }

        Some(Commands::Balance { address }) => {
            let blockchain = Blockchain::new()?;
            let balance = blockchain.get_balance(&address);
            println!("💰 Balance for {}: {}", address, balance);
        }

        Some(Commands::ListWallets) => {
            let storage = BlockchainStorage::create_file("blockchain.db").await?;
            let wallets = storage.list_wallets().await?;

            println!("💳 Wallets:");
            for wallet in wallets {
                println!("  {} - {} (created: {})", wallet.name, wallet.address, wallet.created_at);
            }
        }

        Some(Commands::DeployContract { name: _, wasm_file: _, owner: _ }) => {
            println!("📝 Contract deployment is not implemented in CLI mode. Use the API.");
        }

        Some(Commands::CallContract { contract_id: _, function: _, caller: _ }) => {
            println!("📞 Contract calling is not implemented in CLI mode. Use the API.");
        }
    }

    Ok(())
}

async fn start_full_node(api_port: u16, p2p_port: u16, database_path: &str) -> Result<()> {
    let storage = BlockchainStorage::create_file(database_path).await?;
    let blockchain = Arc::new(RwLock::new(Blockchain::new()?));
    let contract_engine = Arc::new(RwLock::new(ContractEngine::new()?));
    let mining_stats = Arc::new(RwLock::new(
        storage.load_mining_stats().await?.unwrap_or_default()
    ));
    let network_stats = Arc::new(RwLock::new(NetworkStats::default()));
    let wallets = Arc::new(RwLock::new(HashMap::new()));

    // Load existing wallets
    let wallet_list = storage.list_wallets().await?;
    for wallet_info in wallet_list {
        if let Ok(Some(wallet)) = storage.load_wallet(&wallet_info.address).await {
            wallets.write().await.insert(wallet.address(), wallet);
        }
    }

    let api_state = ApiState {
        blockchain: blockchain.clone(),
        storage: Arc::new(storage),
        contract_engine,
        mining_stats,
        network_stats: network_stats.clone(),
        wallets,
    };

    // Start P2P network
    let network_config = NetworkConfig {
        listen_port: p2p_port,
        ..Default::default()
    };

    let (mut p2p_node, mut event_receiver) = P2PNode::new(network_config).await?;

    // Spawn P2P network task
    tokio::spawn(async move {
        p2p_node.run().await;
    });

    // Spawn event handler
    tokio::spawn(async move {
        while let Some(event) = event_receiver.recv().await {
            info!("P2P Event: {:?}", event);
            // Handle P2P events here
        }
    });

    // Start API server
    start_server(api_state, api_port).await?;

    Ok(())
}

async fn start_api_server(port: u16, database_path: &str) -> Result<()> {
    let storage = BlockchainStorage::create_file(database_path).await?;
    let blockchain = Arc::new(RwLock::new(Blockchain::new()?));
    let contract_engine = Arc::new(RwLock::new(ContractEngine::new()?));
    let mining_stats = Arc::new(RwLock::new(
        storage.load_mining_stats().await?.unwrap_or_default()
    ));
    let network_stats = Arc::new(RwLock::new(NetworkStats::default()));
    let wallets = Arc::new(RwLock::new(HashMap::new()));

    let api_state = ApiState {
        blockchain,
        storage: Arc::new(storage),
        contract_engine,
        mining_stats,
        network_stats,
        wallets,
    };

    start_server(api_state, port).await?;
    Ok(())
}