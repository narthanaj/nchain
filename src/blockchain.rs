use crate::block::Block;
use crate::errors::{BlockchainError, Result};
use crate::poh::PohRecorder;
use crate::transaction::Transaction;

#[derive(Debug)]
pub struct Blockchain {
    chain: Vec<Block>,
    poh_recorder: PohRecorder,
}

impl Blockchain {
    pub fn new() -> Result<Self> {
        let mut poh_recorder = PohRecorder::new();
        let mut genesis_block = Block::genesis()?;

        let transaction_data = genesis_block.transaction_data()?;
        genesis_block.poh_hash = poh_recorder.record(&transaction_data);
        genesis_block.hash = genesis_block.calculate_hash();

        Ok(Blockchain {
            chain: vec![genesis_block],
            poh_recorder,
        })
    }

    pub fn add_block(&mut self, transactions: Vec<Transaction>) -> Result<()> {
        if transactions.is_empty() {
            return Err(BlockchainError::InvalidBlock {
                message: "Cannot create block with no transactions".to_string(),
            });
        }

        let (previous_index, previous_hash) = {
            let previous_block = self.get_latest_block()?;
            (previous_block.index, previous_block.hash.clone())
        };

        let transaction_data = transactions
            .iter()
            .map(|tx| tx.serialize())
            .collect::<Result<Vec<String>>>()?
            .join(",");

        let poh_hash = self.poh_recorder.record(&transaction_data);

        let new_block = Block::new(
            previous_index + 1,
            transactions,
            previous_hash,
            poh_hash,
        );

        new_block.is_valid()?;
        self.chain.push(new_block);

        Ok(())
    }

    pub fn get_latest_block(&self) -> Result<&Block> {
        self.chain.last().ok_or(BlockchainError::EmptyBlockchain)
    }

    pub fn get_block(&self, index: u64) -> Option<&Block> {
        self.chain.get(index as usize)
    }

    pub fn chain(&self) -> &[Block] {
        &self.chain
    }

    pub fn len(&self) -> usize {
        self.chain.len()
    }

    pub fn is_empty(&self) -> bool {
        self.chain.is_empty()
    }

    pub fn is_chain_valid(&self) -> Result<()> {
        if self.chain.is_empty() {
            return Err(BlockchainError::ChainValidation {
                message: "Blockchain is empty".to_string(),
            });
        }

        for (i, block) in self.chain.iter().enumerate() {
            block.is_valid()?;

            if i > 0 {
                let previous_block = &self.chain[i - 1];

                if block.previous_hash != previous_block.hash {
                    return Err(BlockchainError::ChainValidation {
                        message: format!(
                            "Block {} has invalid previous hash. Expected: {}, Got: {}",
                            i, previous_block.hash, block.previous_hash
                        ),
                    });
                }

                if block.index != previous_block.index + 1 {
                    return Err(BlockchainError::ChainValidation {
                        message: format!(
                            "Block {} has invalid index. Expected: {}, Got: {}",
                            i,
                            previous_block.index + 1,
                            block.index
                        ),
                    });
                }
            }
        }

        Ok(())
    }

    pub fn get_balance(&self, address: &str) -> f64 {
        let mut balance = 0.0;

        for block in &self.chain {
            for transaction in &block.transactions {
                if transaction.to == address {
                    balance += transaction.amount;
                }
                if transaction.from == address && transaction.from != "genesis" {
                    balance -= transaction.amount;
                }
            }
        }

        balance
    }

    pub fn poh_tick_count(&self) -> u64 {
        self.poh_recorder.tick_count()
    }
}

impl Default for Blockchain {
    fn default() -> Self {
        Self::new().expect("Failed to create default blockchain")
    }
}