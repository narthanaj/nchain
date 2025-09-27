use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::errors::{BlockchainError, Result};
use crate::transaction::Transaction;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Block {
    pub index: u64,
    pub timestamp: DateTime<Utc>,
    pub transactions: Vec<Transaction>,
    pub previous_hash: String,
    pub hash: String,
    pub poh_hash: String,
    pub nonce: u64,
    pub difficulty: u32,
    pub miner: Option<String>,
}

impl Block {
    pub fn new(
        index: u64,
        transactions: Vec<Transaction>,
        previous_hash: String,
        poh_hash: String,
    ) -> Self {
        let miner = transactions
            .iter()
            .find(|tx| tx.is_coinbase())
            .map(|tx| tx.to.clone());

        let mut block = Block {
            index,
            timestamp: Utc::now(),
            transactions,
            previous_hash,
            hash: String::new(),
            poh_hash,
            nonce: 0,
            difficulty: 4,
            miner,
        };

        block.hash = block.calculate_hash();
        block
    }

    pub fn with_difficulty(
        index: u64,
        transactions: Vec<Transaction>,
        previous_hash: String,
        poh_hash: String,
        difficulty: u32,
    ) -> Self {
        let miner = transactions
            .iter()
            .find(|tx| tx.is_coinbase())
            .map(|tx| tx.to.clone());

        let mut block = Block {
            index,
            timestamp: Utc::now(),
            transactions,
            previous_hash,
            hash: String::new(),
            poh_hash,
            nonce: 0,
            difficulty,
            miner,
        };

        block.hash = block.calculate_hash();
        block
    }

    pub fn genesis() -> Result<Self> {
        let genesis_transaction = Transaction::genesis_transaction();
        let mut block = Block {
            index: 0,
            timestamp: Utc::now(),
            transactions: vec![genesis_transaction],
            previous_hash: "0".repeat(64),
            hash: String::new(),
            poh_hash: String::new(),
            nonce: 0,
            difficulty: 1,
            miner: None,
        };

        block.hash = block.calculate_hash();
        Ok(block)
    }

    pub fn calculate_hash(&self) -> String {
        let mut block_copy = self.clone();
        block_copy.hash = String::new();

        let serialized = serde_json::to_string(&block_copy)
            .expect("Block serialization should never fail");

        let mut hasher = Sha256::new();
        hasher.update(serialized.as_bytes());
        format!("{:x}", hasher.finalize())
    }

    pub fn is_valid(&self) -> Result<()> {
        if self.hash != self.calculate_hash() {
            return Err(BlockchainError::InvalidBlock {
                message: "Block hash is invalid".to_string(),
            });
        }

        if self.transactions.is_empty() {
            return Err(BlockchainError::InvalidBlock {
                message: "Block must contain at least one transaction".to_string(),
            });
        }

        for transaction in &self.transactions {
            if transaction.from.trim().is_empty() || transaction.to.trim().is_empty() {
                return Err(BlockchainError::InvalidBlock {
                    message: "Transaction contains empty addresses".to_string(),
                });
            }
        }

        Ok(())
    }

    pub fn transaction_data(&self) -> Result<String> {
        let tx_strings: Result<Vec<String>> = self
            .transactions
            .iter()
            .map(|tx| tx.serialize())
            .collect();

        Ok(tx_strings?.join(","))
    }
}