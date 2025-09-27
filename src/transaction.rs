use serde::{Deserialize, Serialize};
use crate::errors::{BlockchainError, Result};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Transaction {
    pub from: String,
    pub to: String,
    pub amount: f64,
    pub data: Option<String>,
}

impl Transaction {
    pub fn new(from: String, to: String, amount: f64, data: Option<String>) -> Result<Self> {
        if from.trim().is_empty() {
            return Err(BlockchainError::InvalidTransaction {
                message: "From address cannot be empty".to_string(),
            });
        }

        if to.trim().is_empty() {
            return Err(BlockchainError::InvalidTransaction {
                message: "To address cannot be empty".to_string(),
            });
        }

        if amount < 0.0 {
            return Err(BlockchainError::InvalidTransaction {
                message: "Amount cannot be negative".to_string(),
            });
        }

        Ok(Transaction {
            from,
            to,
            amount,
            data,
        })
    }

    pub fn genesis_transaction() -> Self {
        Transaction {
            from: "genesis".to_string(),
            to: "genesis".to_string(),
            amount: 0.0,
            data: Some("Genesis transaction".to_string()),
        }
    }

    pub fn serialize(&self) -> Result<String> {
        serde_json::to_string(self).map_err(BlockchainError::from)
    }
}