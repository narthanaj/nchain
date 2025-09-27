use serde::{Deserialize, Serialize};
use crate::crypto::{DigitalSignature, PublicKey};
use crate::errors::{BlockchainError, Result};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Transaction {
    pub id: String,
    pub from: String,
    pub to: String,
    pub amount: f64,
    pub data: Option<String>,
    pub timestamp: DateTime<Utc>,
    pub signature: Option<DigitalSignature>,
    pub from_public_key: Option<PublicKey>,
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
            id: Uuid::new_v4().to_string(),
            from,
            to,
            amount,
            data,
            timestamp: Utc::now(),
            signature: None,
            from_public_key: None,
        })
    }

    pub fn new_signed(
        from: String,
        to: String,
        amount: f64,
        data: Option<String>,
        signature: DigitalSignature,
        from_public_key: PublicKey,
    ) -> Result<Self> {
        let mut transaction = Self::new(from, to, amount, data)?;
        transaction.signature = Some(signature);
        transaction.from_public_key = Some(from_public_key);
        Ok(transaction)
    }

    pub fn genesis_transaction() -> Self {
        Transaction {
            id: "genesis".to_string(),
            from: "genesis".to_string(),
            to: "genesis".to_string(),
            amount: 0.0,
            data: Some("Genesis transaction".to_string()),
            timestamp: Utc::now(),
            signature: None,
            from_public_key: None,
        }
    }

    pub fn signable_data(&self) -> Result<Vec<u8>> {
        let signable = SignableTransaction {
            id: &self.id,
            from: &self.from,
            to: &self.to,
            amount: self.amount,
            data: self.data.as_ref(),
            timestamp: self.timestamp,
        };

        serde_json::to_vec(&signable).map_err(BlockchainError::from)
    }

    pub fn verify_signature(&self) -> bool {
        if let (Some(signature), Some(public_key)) = (&self.signature, &self.from_public_key) {
            if let Ok(signable_data) = self.signable_data() {
                return public_key.verify(&signable_data, signature);
            }
        }

        self.from == "genesis" || self.from == "miner"
    }

    pub fn serialize(&self) -> Result<String> {
        serde_json::to_string(self).map_err(BlockchainError::from)
    }

    pub fn is_coinbase(&self) -> bool {
        self.from == "miner"
    }
}

#[derive(Serialize)]
struct SignableTransaction<'a> {
    id: &'a str,
    from: &'a str,
    to: &'a str,
    amount: f64,
    data: Option<&'a String>,
    timestamp: DateTime<Utc>,
}