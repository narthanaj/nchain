use sha2::{Digest, Sha256};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PohRecorder {
    current_hash: String,
    iterations: u64,
    tick_count: u64,
}

impl PohRecorder {
    const DEFAULT_ITERATIONS: u64 = 1000;
    const GENESIS_SEED: &'static str = "poh-genesis-seed-solana-inspired";

    pub fn new() -> Self {
        Self::with_iterations(Self::DEFAULT_ITERATIONS)
    }

    pub fn with_iterations(iterations: u64) -> Self {
        PohRecorder {
            current_hash: Self::GENESIS_SEED.to_string(),
            iterations,
            tick_count: 0,
        }
    }

    pub fn record(&mut self, data: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(self.current_hash.as_bytes());
        hasher.update(data.as_bytes());

        let mut current_hash = hasher.finalize();

        for _ in 0..self.iterations {
            let mut iteration_hasher = Sha256::new();
            iteration_hasher.update(current_hash);
            current_hash = iteration_hasher.finalize();
        }

        let final_hash = format!("{:x}", current_hash);
        self.current_hash = final_hash.clone();
        self.tick_count += 1;

        final_hash
    }

    pub fn tick_count(&self) -> u64 {
        self.tick_count
    }

    pub fn current_hash(&self) -> &str {
        &self.current_hash
    }

    pub fn verify_sequence(&self, previous_hash: &str, data: &str, expected_hash: &str) -> bool {
        let mut hasher = Sha256::new();
        hasher.update(previous_hash.as_bytes());
        hasher.update(data.as_bytes());

        let mut current_hash = hasher.finalize();

        for _ in 0..self.iterations {
            let mut iteration_hasher = Sha256::new();
            iteration_hasher.update(current_hash);
            current_hash = iteration_hasher.finalize();
        }

        let computed_hash = format!("{:x}", current_hash);
        computed_hash == expected_hash
    }
}

impl Default for PohRecorder {
    fn default() -> Self {
        Self::new()
    }
}