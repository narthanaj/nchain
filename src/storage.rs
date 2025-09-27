use crate::block::Block;
use crate::crypto::Wallet;
use crate::errors::{BlockchainError, Result};
use crate::mining::MiningStats;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{Row, SqlitePool};
use std::path::Path;
use tracing::{debug, info};

#[derive(Clone)]
pub struct BlockchainStorage {
    pool: SqlitePool,
}

impl BlockchainStorage {
    pub async fn new(database_url: &str) -> Result<Self> {
        info!("Connecting to database: {}", database_url);

        let pool = SqlitePool::connect(database_url)
            .await
            .map_err(|e| BlockchainError::Io(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Database connection failed: {}", e),
            )))?;

        let storage = Self { pool };
        storage.run_migrations().await?;

        info!("Database connected and migrations completed");
        Ok(storage)
    }

    pub async fn create_in_memory() -> Result<Self> {
        Self::new("sqlite::memory:").await
    }

    pub async fn create_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let url = format!("sqlite:{}", path.as_ref().display());
        Self::new(&url).await
    }

    async fn run_migrations(&self) -> Result<()> {
        debug!("Running database migrations");

        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS blocks (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                index_num INTEGER UNIQUE NOT NULL,
                timestamp TEXT NOT NULL,
                previous_hash TEXT NOT NULL,
                hash TEXT NOT NULL,
                poh_hash TEXT NOT NULL,
                nonce INTEGER NOT NULL,
                difficulty INTEGER NOT NULL,
                miner TEXT,
                data TEXT NOT NULL
            )
            "#,
        )
        .execute(&self.pool)
        .await
        .map_err(|e| BlockchainError::Io(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Failed to create blocks table: {}", e),
        )))?;

        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS transactions (
                id TEXT PRIMARY KEY,
                block_index INTEGER NOT NULL,
                from_address TEXT NOT NULL,
                to_address TEXT NOT NULL,
                amount REAL NOT NULL,
                data TEXT,
                timestamp TEXT NOT NULL,
                signature TEXT,
                from_public_key TEXT,
                FOREIGN KEY (block_index) REFERENCES blocks (index_num)
            )
            "#,
        )
        .execute(&self.pool)
        .await
        .map_err(|e| BlockchainError::Io(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Failed to create transactions table: {}", e),
        )))?;

        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS wallets (
                address TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                public_key TEXT NOT NULL,
                private_key TEXT NOT NULL,
                created_at TEXT NOT NULL
            )
            "#,
        )
        .execute(&self.pool)
        .await
        .map_err(|e| BlockchainError::Io(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Failed to create wallets table: {}", e),
        )))?;

        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS mining_stats (
                id INTEGER PRIMARY KEY CHECK (id = 1),
                total_blocks_mined INTEGER NOT NULL,
                total_mining_time_secs INTEGER NOT NULL,
                average_hash_rate INTEGER NOT NULL,
                total_rewards REAL NOT NULL,
                current_difficulty INTEGER NOT NULL
            )
            "#,
        )
        .execute(&self.pool)
        .await
        .map_err(|e| BlockchainError::Io(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Failed to create mining_stats table: {}", e),
        )))?;

        debug!("Database migrations completed successfully");
        Ok(())
    }

    pub async fn save_block(&self, block: &Block) -> Result<()> {
        debug!("Saving block #{} to database", block.index);

        let block_data = serde_json::to_string(block)
            .map_err(|e| BlockchainError::Serialization(e))?;

        let mut tx = self.pool.begin().await.map_err(|e| {
            BlockchainError::Io(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Failed to begin transaction: {}", e),
            ))
        })?;

        sqlx::query(
            r#"
            INSERT INTO blocks (index_num, timestamp, previous_hash, hash, poh_hash, nonce, difficulty, miner, data)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(block.index as i64)
        .bind(block.timestamp.to_rfc3339())
        .bind(&block.previous_hash)
        .bind(&block.hash)
        .bind(&block.poh_hash)
        .bind(block.nonce as i64)
        .bind(block.difficulty as i64)
        .bind(&block.miner)
        .bind(&block_data)
        .execute(&mut *tx)
        .await
        .map_err(|e| BlockchainError::Io(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Failed to insert block: {}", e),
        )))?;

        for transaction in &block.transactions {
            sqlx::query(
                r#"
                INSERT INTO transactions (id, block_index, from_address, to_address, amount, data, timestamp, signature, from_public_key)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
                "#,
            )
            .bind(&transaction.id)
            .bind(block.index as i64)
            .bind(&transaction.from)
            .bind(&transaction.to)
            .bind(transaction.amount)
            .bind(&transaction.data)
            .bind(transaction.timestamp.to_rfc3339())
            .bind(transaction.signature.as_ref().map(|s| s.to_string()))
            .bind(transaction.from_public_key.as_ref().map(|pk| pk.to_string()))
            .execute(&mut *tx)
            .await
            .map_err(|e| BlockchainError::Io(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Failed to insert transaction: {}", e),
            )))?;
        }

        tx.commit().await.map_err(|e| {
            BlockchainError::Io(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Failed to commit transaction: {}", e),
            ))
        })?;

        info!("Block #{} saved to database successfully", block.index);
        Ok(())
    }

    pub async fn load_block(&self, index: u64) -> Result<Option<Block>> {
        debug!("Loading block #{} from database", index);

        let row = sqlx::query("SELECT data FROM blocks WHERE index_num = ?")
            .bind(index as i64)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| BlockchainError::Io(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Failed to load block: {}", e),
            )))?;

        if let Some(row) = row {
            let block_data: String = row.get("data");
            let block: Block = serde_json::from_str(&block_data)
                .map_err(|e| BlockchainError::Serialization(e))?;
            Ok(Some(block))
        } else {
            Ok(None)
        }
    }

    pub async fn load_all_blocks(&self) -> Result<Vec<Block>> {
        debug!("Loading all blocks from database");

        let rows = sqlx::query("SELECT data FROM blocks ORDER BY index_num")
            .fetch_all(&self.pool)
            .await
            .map_err(|e| BlockchainError::Io(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Failed to load blocks: {}", e),
            )))?;

        let mut blocks = Vec::new();
        for row in rows {
            let block_data: String = row.get("data");
            let block: Block = serde_json::from_str(&block_data)
                .map_err(|e| BlockchainError::Serialization(e))?;
            blocks.push(block);
        }

        info!("Loaded {} blocks from database", blocks.len());
        Ok(blocks)
    }

    pub async fn get_latest_block_index(&self) -> Result<Option<u64>> {
        let row = sqlx::query("SELECT MAX(index_num) as max_index FROM blocks")
            .fetch_one(&self.pool)
            .await
            .map_err(|e| BlockchainError::Io(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Failed to get latest block index: {}", e),
            )))?;

        let max_index: Option<i64> = row.get("max_index");
        Ok(max_index.map(|i| i as u64))
    }

    pub async fn save_wallet(&self, wallet: &Wallet) -> Result<()> {
        debug!("Saving wallet '{}' to database", wallet.name);

        sqlx::query(
            r#"
            INSERT OR REPLACE INTO wallets (address, name, public_key, private_key, created_at)
            VALUES (?, ?, ?, ?, ?)
            "#,
        )
        .bind(&wallet.address())
        .bind(&wallet.name)
        .bind(&wallet.keypair.public_key().to_string())
        .bind(hex::encode(wallet.keypair.to_private_key_bytes()))
        .bind(Utc::now().to_rfc3339())
        .execute(&self.pool)
        .await
        .map_err(|e| BlockchainError::Io(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Failed to save wallet: {}", e),
        )))?;

        info!("Wallet '{}' saved to database", wallet.name);
        Ok(())
    }

    pub async fn load_wallet(&self, address: &str) -> Result<Option<Wallet>> {
        debug!("Loading wallet with address '{}' from database", address);

        let row = sqlx::query("SELECT name, private_key FROM wallets WHERE address = ?")
            .bind(address)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| BlockchainError::Io(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Failed to load wallet: {}", e),
            )))?;

        if let Some(row) = row {
            let name: String = row.get("name");
            let private_key_hex: String = row.get("private_key");
            let private_key = hex::decode(private_key_hex)
                .map_err(|e| BlockchainError::InvalidTransaction {
                    message: format!("Invalid private key format: {}", e),
                })?;

            let wallet = Wallet::from_private_key(name, &private_key)?;
            Ok(Some(wallet))
        } else {
            Ok(None)
        }
    }

    pub async fn list_wallets(&self) -> Result<Vec<WalletInfo>> {
        debug!("Loading all wallets from database");

        let rows = sqlx::query("SELECT address, name, created_at FROM wallets ORDER BY created_at")
            .fetch_all(&self.pool)
            .await
            .map_err(|e| BlockchainError::Io(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Failed to load wallets: {}", e),
            )))?;

        let mut wallets = Vec::new();
        for row in rows {
            let address: String = row.get("address");
            let name: String = row.get("name");
            let created_at_str: String = row.get("created_at");
            let created_at = DateTime::parse_from_rfc3339(&created_at_str)
                .map_err(|e| BlockchainError::InvalidTransaction {
                    message: format!("Invalid timestamp format: {}", e),
                })?
                .with_timezone(&Utc);

            wallets.push(WalletInfo {
                address,
                name,
                created_at,
            });
        }

        Ok(wallets)
    }

    pub async fn save_mining_stats(&self, stats: &MiningStats) -> Result<()> {
        debug!("Saving mining stats to database");

        sqlx::query(
            r#"
            INSERT OR REPLACE INTO mining_stats (id, total_blocks_mined, total_mining_time_secs, average_hash_rate, total_rewards, current_difficulty)
            VALUES (1, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(stats.total_blocks_mined as i64)
        .bind(stats.total_mining_time.as_secs() as i64)
        .bind(stats.average_hash_rate as i64)
        .bind(stats.total_rewards)
        .bind(stats.current_difficulty as i64)
        .execute(&self.pool)
        .await
        .map_err(|e| BlockchainError::Io(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Failed to save mining stats: {}", e),
        )))?;

        Ok(())
    }

    pub async fn load_mining_stats(&self) -> Result<Option<MiningStats>> {
        debug!("Loading mining stats from database");

        let row = sqlx::query("SELECT * FROM mining_stats WHERE id = 1")
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| BlockchainError::Io(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Failed to load mining stats: {}", e),
            )))?;

        if let Some(row) = row {
            let total_blocks_mined: i64 = row.get("total_blocks_mined");
            let total_mining_time_secs: i64 = row.get("total_mining_time_secs");
            let average_hash_rate: i64 = row.get("average_hash_rate");
            let total_rewards: f64 = row.get("total_rewards");
            let current_difficulty: i64 = row.get("current_difficulty");

            Ok(Some(MiningStats {
                total_blocks_mined: total_blocks_mined as u64,
                total_mining_time: std::time::Duration::from_secs(total_mining_time_secs as u64),
                average_hash_rate: average_hash_rate as u64,
                total_rewards,
                current_difficulty: current_difficulty as u32,
            }))
        } else {
            Ok(None)
        }
    }

    pub async fn get_transaction_count(&self) -> Result<u64> {
        let row = sqlx::query("SELECT COUNT(*) as count FROM transactions")
            .fetch_one(&self.pool)
            .await
            .map_err(|e| BlockchainError::Io(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Failed to get transaction count: {}", e),
            )))?;

        let count: i64 = row.get("count");
        Ok(count as u64)
    }

    pub async fn close(&self) {
        self.pool.close().await;
        info!("Database connection closed");
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WalletInfo {
    pub address: String,
    pub name: String,
    pub created_at: DateTime<Utc>,
}