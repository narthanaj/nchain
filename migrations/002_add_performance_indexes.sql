-- Additional performance indexes and optimizations
-- This migration adds more indexes for better query performance

-- Additional indexes for transactions
CREATE INDEX IF NOT EXISTS idx_transactions_amount ON transactions(amount);
CREATE INDEX IF NOT EXISTS idx_transactions_is_coinbase ON transactions(is_coinbase);
CREATE INDEX IF NOT EXISTS idx_transactions_composite_address_timestamp ON transactions(from_address, timestamp);

-- Composite indexes for complex queries
CREATE INDEX IF NOT EXISTS idx_blocks_composite_index_timestamp ON blocks(block_index, timestamp);
CREATE INDEX IF NOT EXISTS idx_blocks_composite_miner_timestamp ON blocks(miner, timestamp) WHERE miner IS NOT NULL;

-- Contract-related indexes
CREATE INDEX IF NOT EXISTS idx_contract_states_updated_at ON contract_states(updated_at);
CREATE INDEX IF NOT EXISTS idx_contracts_active ON smart_contracts(is_active) WHERE is_active = TRUE;

-- Performance view for transaction history
CREATE VIEW IF NOT EXISTS transaction_history AS
SELECT
    t.id,
    t.from_address,
    t.to_address,
    t.amount,
    t.timestamp,
    t.is_coinbase,
    b.block_index,
    b.hash as block_hash
FROM transactions t
LEFT JOIN blocks b ON t.block_id = b.id
ORDER BY t.timestamp DESC;

-- View for address transaction summary
CREATE VIEW IF NOT EXISTS address_transaction_summary AS
SELECT
    address,
    COUNT(*) as total_transactions,
    SUM(CASE WHEN from_address = address THEN -amount ELSE amount END) as net_amount,
    SUM(CASE WHEN from_address = address THEN amount ELSE 0 END) as total_sent,
    SUM(CASE WHEN to_address = address THEN amount ELSE 0 END) as total_received,
    MIN(timestamp) as first_transaction,
    MAX(timestamp) as last_transaction
FROM (
    SELECT from_address as address, amount, timestamp FROM transactions
    UNION ALL
    SELECT to_address as address, amount, timestamp FROM transactions
) combined
GROUP BY address;

-- Update system metadata
INSERT OR REPLACE INTO system_metadata (key, value) VALUES
    ('schema_version', '2'),
    ('performance_indexes_added', 'true'),
    ('updated_at', datetime('now'));