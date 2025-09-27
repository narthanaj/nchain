use crate::block::Block;
use crate::crypto::Wallet;
use crate::errors::{BlockchainError, Result};
use crate::transaction::Transaction;
use serde::{Deserialize, Serialize};
use std::time::{Duration, Instant};
use tracing::{debug, info};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiningConfig {
    pub difficulty: u32,
    pub block_reward: f64,
    pub max_block_time: Duration,
    pub difficulty_adjustment_interval: u64,
    pub target_block_time: Duration,
}

impl Default for MiningConfig {
    fn default() -> Self {
        MiningConfig {
            difficulty: 4,
            block_reward: 50.0,
            max_block_time: Duration::from_secs(600), // 10 minutes max
            difficulty_adjustment_interval: 10,       // Adjust every 10 blocks
            target_block_time: Duration::from_secs(60), // Target 1 minute per block
        }
    }
}

#[derive(Debug, Clone)]
pub struct MiningResult {
    pub block: Block,
    pub mining_time: Duration,
    pub hash_rate: u64,
    pub nonce: u64,
}

pub struct Miner {
    config: MiningConfig,
    wallet: Wallet,
}

impl Miner {
    pub fn new(config: MiningConfig, wallet: Wallet) -> Self {
        Miner { config, wallet }
    }

    pub fn mine_block(
        &self,
        index: u64,
        transactions: Vec<Transaction>,
        previous_hash: String,
        poh_hash: String,
    ) -> Result<MiningResult> {
        info!("Starting to mine block #{}", index);

        let mut block_transactions = transactions;

        let coinbase_transaction = Transaction::new_signed(
            "miner".to_string(),
            self.wallet.address(),
            self.config.block_reward,
            Some("Block reward".to_string()),
            self.wallet.sign_transaction(b"coinbase"),
            self.wallet.keypair.public_key().clone(),
        )?;

        block_transactions.insert(0, coinbase_transaction);

        let mut block = Block::new(index, block_transactions, previous_hash, poh_hash);

        let start_time = Instant::now();
        let mut nonce = 0u64;
        let target = self.calculate_target(self.config.difficulty);

        loop {
            block.nonce = nonce;
            let hash = block.calculate_hash();

            if self.hash_meets_difficulty(&hash, &target) {
                let mining_time = start_time.elapsed();
                let hash_rate = if mining_time.as_secs() > 0 {
                    nonce / mining_time.as_secs()
                } else {
                    nonce
                };

                block.hash = hash;

                info!(
                    "Block mined! Nonce: {}, Time: {:?}, Hash rate: {} H/s",
                    nonce, mining_time, hash_rate
                );

                return Ok(MiningResult {
                    block,
                    mining_time,
                    hash_rate,
                    nonce,
                });
            }

            nonce += 1;

            if start_time.elapsed() > self.config.max_block_time {
                return Err(BlockchainError::InvalidBlock {
                    message: "Mining timeout exceeded".to_string(),
                });
            }

            if nonce % 100_000 == 0 {
                debug!("Mining progress: nonce = {}, time = {:?}", nonce, start_time.elapsed());
            }
        }
    }

    pub fn calculate_difficulty_adjustment(
        &self,
        blocks: &[Block],
        current_difficulty: u32,
    ) -> u32 {
        if blocks.len() < self.config.difficulty_adjustment_interval as usize {
            return current_difficulty;
        }

        let recent_blocks = &blocks[blocks.len() - self.config.difficulty_adjustment_interval as usize..];

        let time_taken = if recent_blocks.len() >= 2 {
            let start_time = recent_blocks[0].timestamp;
            let end_time = recent_blocks[recent_blocks.len() - 1].timestamp;
            end_time.signed_duration_since(start_time)
        } else {
            return current_difficulty;
        };

        let expected_time = self.config.target_block_time * self.config.difficulty_adjustment_interval as u32;
        let expected_duration = chrono::Duration::from_std(expected_time).unwrap_or_default();

        let ratio = time_taken.num_seconds() as f64 / expected_duration.num_seconds() as f64;

        let new_difficulty = if ratio < 0.5 {
            current_difficulty + 1
        } else if ratio > 2.0 {
            if current_difficulty > 1 {
                current_difficulty - 1
            } else {
                1
            }
        } else {
            current_difficulty
        };

        info!(
            "Difficulty adjustment: {} -> {} (ratio: {:.2})",
            current_difficulty, new_difficulty, ratio
        );

        new_difficulty
    }

    fn calculate_target(&self, difficulty: u32) -> String {
        "0".repeat(difficulty as usize)
    }

    fn hash_meets_difficulty(&self, hash: &str, target: &str) -> bool {
        hash.starts_with(target)
    }

    pub fn estimate_mining_time(&self, difficulty: u32, hash_rate: u64) -> Duration {
        if hash_rate == 0 {
            return Duration::from_secs(u64::MAX);
        }

        let target_hashes = 16u64.pow(difficulty);
        let expected_time = target_hashes / hash_rate;

        Duration::from_secs(expected_time)
    }

    pub fn config(&self) -> &MiningConfig {
        &self.config
    }

    pub fn wallet(&self) -> &Wallet {
        &self.wallet
    }

    pub fn set_difficulty(&mut self, difficulty: u32) {
        self.config.difficulty = difficulty;
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiningStats {
    pub total_blocks_mined: u64,
    pub total_mining_time: Duration,
    pub average_hash_rate: u64,
    pub total_rewards: f64,
    pub current_difficulty: u32,
}

impl Default for MiningStats {
    fn default() -> Self {
        MiningStats {
            total_blocks_mined: 0,
            total_mining_time: Duration::from_secs(0),
            average_hash_rate: 0,
            total_rewards: 0.0,
            current_difficulty: 4,
        }
    }
}

impl MiningStats {
    pub fn update(&mut self, result: &MiningResult, difficulty: u32) {
        self.total_blocks_mined += 1;
        self.total_mining_time += result.mining_time;
        self.total_rewards += result.block.transactions
            .iter()
            .filter(|tx| tx.is_coinbase())
            .map(|tx| tx.amount)
            .sum::<f64>();
        self.current_difficulty = difficulty;

        if self.total_mining_time.as_secs() > 0 {
            self.average_hash_rate = result.nonce / self.total_mining_time.as_secs();
        }
    }
}