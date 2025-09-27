use crate::errors::{BlockchainError, Result};
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockchainConfig {
    pub node: NodeConfig,
    pub database: DatabaseConfig,
    pub mining: MiningConfig,
    pub network: NetworkConfig,
    pub api: ApiConfig,
    pub contracts: ContractsConfig,
    pub logging: LoggingConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeConfig {
    pub name: String,
    pub data_dir: String,
    pub genesis_block_reward: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
    pub connection_timeout_secs: u64,
    pub enable_migrations: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiningConfig {
    pub difficulty: u32,
    pub block_reward: f64,
    pub max_block_time_secs: u64,
    pub difficulty_adjustment_interval: u64,
    pub target_block_time_secs: u64,
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkConfig {
    pub listen_port: u16,
    pub bootstrap_peers: Vec<String>,
    pub max_peers: usize,
    pub sync_interval_secs: u64,
    pub connection_timeout_secs: u64,
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiConfig {
    pub enabled: bool,
    pub bind_address: String,
    pub port: u16,
    pub cors_enabled: bool,
    pub cors_origins: Vec<String>,
    pub rate_limit_requests_per_minute: u32,
    pub request_timeout_secs: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractsConfig {
    pub enabled: bool,
    pub max_memory_mb: usize,
    pub execution_timeout_secs: u64,
    pub max_gas_limit: u64,
    pub gas_price: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfig {
    pub level: String,
    pub format: String,
    pub output: LogOutput,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LogOutput {
    Console,
    File { path: String },
    Both { path: String },
}

impl Default for BlockchainConfig {
    fn default() -> Self {
        Self {
            node: NodeConfig::default(),
            database: DatabaseConfig::default(),
            mining: MiningConfig::default(),
            network: NetworkConfig::default(),
            api: ApiConfig::default(),
            contracts: ContractsConfig::default(),
            logging: LoggingConfig::default(),
        }
    }
}

impl Default for NodeConfig {
    fn default() -> Self {
        Self {
            name: "blockchain-node".to_string(),
            data_dir: "./data".to_string(),
            genesis_block_reward: 50.0,
        }
    }
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            url: "sqlite:./data/blockchain.db".to_string(),
            max_connections: 10,
            connection_timeout_secs: 30,
            enable_migrations: true,
        }
    }
}

impl Default for MiningConfig {
    fn default() -> Self {
        Self {
            difficulty: 4,
            block_reward: 12.5,
            max_block_time_secs: 300, // 5 minutes
            difficulty_adjustment_interval: 2016, // blocks
            target_block_time_secs: 600, // 10 minutes
            enabled: false,
        }
    }
}

impl Default for NetworkConfig {
    fn default() -> Self {
        Self {
            listen_port: 9000,
            bootstrap_peers: vec![],
            max_peers: 50,
            sync_interval_secs: 30,
            connection_timeout_secs: 10,
            enabled: false,
        }
    }
}

impl Default for ApiConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            bind_address: "127.0.0.1".to_string(),
            port: 8080,
            cors_enabled: true,
            cors_origins: vec!["*".to_string()],
            rate_limit_requests_per_minute: 100,
            request_timeout_secs: 30,
        }
    }
}

impl Default for ContractsConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            max_memory_mb: 16,
            execution_timeout_secs: 30,
            max_gas_limit: 1_000_000,
            gas_price: 0.001,
        }
    }
}

impl Default for LoggingConfig {
    fn default() -> Self {
        Self {
            level: "info".to_string(),
            format: "json".to_string(),
            output: LogOutput::Console,
        }
    }
}

impl BlockchainConfig {
    /// Load configuration from a TOML file
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let content = std::fs::read_to_string(path.as_ref()).map_err(|e| {
            BlockchainError::InvalidBlock {
                message: format!("Failed to read config file: {}", e),
            }
        })?;

        toml::from_str(&content).map_err(|e| BlockchainError::InvalidBlock {
            message: format!("Failed to parse config file: {}", e),
        })
    }

    /// Save configuration to a TOML file
    pub fn to_file<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let content = toml::to_string_pretty(self).map_err(|e| BlockchainError::InvalidBlock {
            message: format!("Failed to serialize config: {}", e),
        })?;

        std::fs::write(path.as_ref(), content).map_err(|e| BlockchainError::InvalidBlock {
            message: format!("Failed to write config file: {}", e),
        })
    }

    /// Validate the configuration
    pub fn validate(&self) -> Result<()> {
        // Validate database URL
        if self.database.url.is_empty() {
            return Err(BlockchainError::InvalidBlock {
                message: "Database URL cannot be empty".to_string(),
            });
        }

        // Validate ports
        if self.api.port == 0 {
            return Err(BlockchainError::InvalidBlock {
                message: "API port cannot be 0".to_string(),
            });
        }

        if self.network.listen_port == 0 {
            return Err(BlockchainError::InvalidBlock {
                message: "Network listen port cannot be 0".to_string(),
            });
        }

        // Validate mining configuration
        if self.mining.difficulty == 0 {
            return Err(BlockchainError::InvalidBlock {
                message: "Mining difficulty cannot be 0".to_string(),
            });
        }

        if self.mining.block_reward < 0.0 {
            return Err(BlockchainError::InvalidBlock {
                message: "Block reward cannot be negative".to_string(),
            });
        }

        // Validate contracts configuration
        if self.contracts.max_memory_mb == 0 {
            return Err(BlockchainError::InvalidBlock {
                message: "Contract max memory cannot be 0".to_string(),
            });
        }

        // Validate data directory
        if self.node.data_dir.is_empty() {
            return Err(BlockchainError::InvalidBlock {
                message: "Data directory cannot be empty".to_string(),
            });
        }

        Ok(())
    }

    /// Convert time-based config values to Duration
    pub fn mining_target_block_time(&self) -> Duration {
        Duration::from_secs(self.mining.target_block_time_secs)
    }

    pub fn mining_max_block_time(&self) -> Duration {
        Duration::from_secs(self.mining.max_block_time_secs)
    }

    pub fn network_sync_interval(&self) -> Duration {
        Duration::from_secs(self.network.sync_interval_secs)
    }

    pub fn network_connection_timeout(&self) -> Duration {
        Duration::from_secs(self.network.connection_timeout_secs)
    }

    pub fn database_connection_timeout(&self) -> Duration {
        Duration::from_secs(self.database.connection_timeout_secs)
    }

    pub fn contracts_execution_timeout(&self) -> Duration {
        Duration::from_secs(self.contracts.execution_timeout_secs)
    }

    pub fn api_request_timeout(&self) -> Duration {
        Duration::from_secs(self.api.request_timeout_secs)
    }

    /// Get contracts max memory in bytes
    pub fn contracts_max_memory_bytes(&self) -> usize {
        self.contracts.max_memory_mb * 1024 * 1024
    }
}