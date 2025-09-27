pub mod block;
pub mod blockchain;
pub mod poh;
pub mod transaction;
pub mod errors;
pub mod cli;
pub mod crypto;
pub mod mining;
pub mod storage;
pub mod network;
pub mod contracts;
pub mod api;
pub mod config;

pub use block::Block;
pub use blockchain::Blockchain;
pub use poh::PohRecorder;
pub use transaction::Transaction;
pub use errors::{BlockchainError, Result};
pub use crypto::{Wallet, KeyPair, PublicKey, DigitalSignature};
pub use mining::{Miner, MiningConfig, MiningStats};
pub use storage::BlockchainStorage;
pub use contracts::{SmartContract, ContractEngine};
pub use api::ApiState;
pub use config::BlockchainConfig;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transaction_creation() {
        let tx = Transaction::new(
            "alice".to_string(),
            "bob".to_string(),
            100.0,
            Some("payment".to_string()),
        ).unwrap();

        assert_eq!(tx.from, "alice");
        assert_eq!(tx.to, "bob");
        assert_eq!(tx.amount, 100.0);
        assert_eq!(tx.data, Some("payment".to_string()));
    }

    #[test]
    fn test_transaction_validation() {
        assert!(Transaction::new("".to_string(), "bob".to_string(), 100.0, None).is_err());
        assert!(Transaction::new("alice".to_string(), "".to_string(), 100.0, None).is_err());
        assert!(Transaction::new("alice".to_string(), "bob".to_string(), -100.0, None).is_err());
    }

    #[test]
    fn test_poh_recorder() {
        let mut poh = PohRecorder::new();
        let hash1 = poh.record("test data 1");
        let hash2 = poh.record("test data 2");

        assert_ne!(hash1, hash2);
        assert_eq!(poh.tick_count(), 2);
        assert!(poh.verify_sequence("poh-genesis-seed-solana-inspired", "test data 1", &hash1));
    }

    #[test]
    fn test_block_creation() {
        let tx = Transaction::genesis_transaction();
        let block = Block::new(0, vec![tx], "0".repeat(64), "poh_hash".to_string());

        assert_eq!(block.index, 0);
        assert_eq!(block.transactions.len(), 1);
        assert!(!block.hash.is_empty());
    }

    #[test]
    fn test_block_validation() {
        let tx = Transaction::genesis_transaction();
        let block = Block::new(0, vec![tx], "0".repeat(64), "poh_hash".to_string());

        assert!(block.is_valid().is_ok());
    }

    #[test]
    fn test_blockchain_creation() {
        let blockchain = Blockchain::new().unwrap();
        assert_eq!(blockchain.len(), 1);
        assert!(!blockchain.is_empty());
        assert!(blockchain.is_chain_valid().is_ok());
    }

    #[test]
    fn test_add_block() {
        let mut blockchain = Blockchain::new().unwrap();
        let tx = Transaction::new(
            "alice".to_string(),
            "bob".to_string(),
            50.0,
            None,
        ).unwrap();

        assert!(blockchain.add_block(vec![tx]).is_ok());
        assert_eq!(blockchain.len(), 2);
        assert!(blockchain.is_chain_valid().is_ok());
    }

    #[test]
    fn test_balance_calculation() {
        let mut blockchain = Blockchain::new().unwrap();

        let tx1 = Transaction::new("genesis".to_string(), "alice".to_string(), 100.0, None).unwrap();
        let tx2 = Transaction::new("alice".to_string(), "bob".to_string(), 30.0, None).unwrap();

        blockchain.add_block(vec![tx1]).unwrap();
        blockchain.add_block(vec![tx2]).unwrap();

        assert_eq!(blockchain.get_balance("alice"), 70.0);
        assert_eq!(blockchain.get_balance("bob"), 30.0);
    }

    #[test]
    fn test_empty_block_rejection() {
        let mut blockchain = Blockchain::new().unwrap();
        assert!(blockchain.add_block(vec![]).is_err());
    }

    #[test]
    fn test_chain_validation() {
        let blockchain = Blockchain::new().unwrap();
        assert!(blockchain.is_chain_valid().is_ok());

        let tx = Transaction::new("alice".to_string(), "bob".to_string(), 50.0, None).unwrap();
        let mut blockchain = blockchain;
        blockchain.add_block(vec![tx]).unwrap();
        assert!(blockchain.is_chain_valid().is_ok());
    }
}