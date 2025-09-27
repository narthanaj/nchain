-- Initial database schema for blockchain
-- This migration creates the basic tables for storing blockchain data

-- Blocks table
CREATE TABLE IF NOT EXISTS blocks (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    block_index INTEGER NOT NULL UNIQUE,
    timestamp TEXT NOT NULL,
    previous_hash TEXT NOT NULL,
    hash TEXT NOT NULL UNIQUE,
    poh_hash TEXT NOT NULL,
    nonce INTEGER NOT NULL DEFAULT 0,
    difficulty INTEGER NOT NULL DEFAULT 4,
    miner TEXT,
    transaction_count INTEGER NOT NULL DEFAULT 0,
    data TEXT NOT NULL, -- JSON serialized block data
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT blocks_index_check CHECK (block_index >= 0),
    CONSTRAINT blocks_difficulty_check CHECK (difficulty > 0)
);

-- Transactions table
CREATE TABLE IF NOT EXISTS transactions (
    id TEXT PRIMARY KEY,
    block_id INTEGER,
    from_address TEXT NOT NULL,
    to_address TEXT NOT NULL,
    amount REAL NOT NULL,
    data TEXT, -- Optional transaction data
    timestamp TEXT NOT NULL,
    signature TEXT, -- Digital signature
    from_public_key TEXT, -- Public key of sender
    is_coinbase BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (block_id) REFERENCES blocks(id) ON DELETE CASCADE,
    CONSTRAINT transactions_amount_check CHECK (amount >= 0)
);

-- Wallets table
CREATE TABLE IF NOT EXISTS wallets (
    address TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    public_key TEXT NOT NULL,
    private_key_encrypted TEXT, -- Encrypted private key (optional)
    balance REAL NOT NULL DEFAULT 0.0,
    transaction_count INTEGER NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT wallets_balance_check CHECK (balance >= 0)
);

-- Smart contracts table
CREATE TABLE IF NOT EXISTS smart_contracts (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    owner TEXT NOT NULL,
    code BLOB NOT NULL, -- WASM bytecode
    abi TEXT, -- JSON ABI
    gas_limit INTEGER NOT NULL DEFAULT 1000000,
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deployed_at TEXT,
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    CONSTRAINT contracts_gas_limit_check CHECK (gas_limit > 0)
);

-- Contract states table
CREATE TABLE IF NOT EXISTS contract_states (
    contract_id TEXT NOT NULL,
    key TEXT NOT NULL,
    value TEXT NOT NULL, -- JSON serialized value
    updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (contract_id, key),
    FOREIGN KEY (contract_id) REFERENCES smart_contracts(id) ON DELETE CASCADE
);

-- Mining statistics table
CREATE TABLE IF NOT EXISTS mining_stats (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    miner_address TEXT NOT NULL,
    blocks_mined INTEGER NOT NULL DEFAULT 0,
    total_rewards REAL NOT NULL DEFAULT 0.0,
    last_block_time TEXT,
    hash_rate REAL DEFAULT 0.0, -- Hashes per second
    updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(miner_address)
);

-- Network peers table
CREATE TABLE IF NOT EXISTS network_peers (
    address TEXT PRIMARY KEY,
    last_seen TEXT NOT NULL,
    connected BOOLEAN NOT NULL DEFAULT FALSE,
    latency_ms INTEGER DEFAULT 0,
    version TEXT,
    user_agent TEXT,
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- System metadata table
CREATE TABLE IF NOT EXISTS system_metadata (
    key TEXT PRIMARY KEY,
    value TEXT NOT NULL,
    updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Indexes for better performance
CREATE INDEX IF NOT EXISTS idx_blocks_hash ON blocks(hash);
CREATE INDEX IF NOT EXISTS idx_blocks_timestamp ON blocks(timestamp);
CREATE INDEX IF NOT EXISTS idx_blocks_miner ON blocks(miner);

CREATE INDEX IF NOT EXISTS idx_transactions_block_id ON transactions(block_id);
CREATE INDEX IF NOT EXISTS idx_transactions_from_address ON transactions(from_address);
CREATE INDEX IF NOT EXISTS idx_transactions_to_address ON transactions(to_address);
CREATE INDEX IF NOT EXISTS idx_transactions_timestamp ON transactions(timestamp);

CREATE INDEX IF NOT EXISTS idx_wallets_balance ON wallets(balance);
CREATE INDEX IF NOT EXISTS idx_wallets_created_at ON wallets(created_at);

CREATE INDEX IF NOT EXISTS idx_contracts_owner ON smart_contracts(owner);
CREATE INDEX IF NOT EXISTS idx_contracts_created_at ON smart_contracts(created_at);

CREATE INDEX IF NOT EXISTS idx_peers_last_seen ON network_peers(last_seen);
CREATE INDEX IF NOT EXISTS idx_peers_connected ON network_peers(connected);

-- Insert initial system metadata
INSERT OR REPLACE INTO system_metadata (key, value) VALUES
    ('schema_version', '1'),
    ('genesis_block_created', 'false'),
    ('node_id', lower(hex(randomblob(16)))),
    ('created_at', datetime('now'));

-- Create views for common queries
CREATE VIEW IF NOT EXISTS block_summary AS
SELECT
    block_index,
    hash,
    timestamp,
    miner,
    transaction_count,
    difficulty
FROM blocks
ORDER BY block_index DESC;

CREATE VIEW IF NOT EXISTS wallet_balances AS
SELECT
    address,
    name,
    balance,
    transaction_count,
    (
        SELECT COUNT(*)
        FROM transactions
        WHERE from_address = wallets.address OR to_address = wallets.address
    ) as total_transactions
FROM wallets
ORDER BY balance DESC;

CREATE VIEW IF NOT EXISTS mining_leaderboard AS
SELECT
    miner_address,
    blocks_mined,
    total_rewards,
    ROUND(total_rewards / NULLIF(blocks_mined, 0), 4) as avg_reward_per_block,
    hash_rate
FROM mining_stats
ORDER BY blocks_mined DESC;