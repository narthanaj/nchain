# Blockchain CLI Documentation

This document provides comprehensive documentation for the Blockchain Node command-line interface.

## Overview

The blockchain CLI provides multiple operation modes and commands for interacting with the blockchain network. You can run nodes, mine blocks, manage wallets, deploy smart contracts, and much more.

## Installation

Build the project first:

```bash
cargo build --release
```

The binary will be available at `target/release/blockchain`.

## Basic Usage

```bash
blockchain [COMMAND] [OPTIONS]
```

## Commands

### Node Operations

#### Start Full Node

Start a complete blockchain node with API server and P2P networking:

```bash
blockchain node [OPTIONS]
```

**Options:**
- `--api-port <PORT>` - API server port (default: 8080)
- `--p2p-port <PORT>` - P2P network port (default: 9000)
- `--database <PATH>` - Database file path (default: blockchain.db)

**Examples:**
```bash
# Start with default settings
blockchain node

# Start with custom ports
blockchain node --api-port 8080 --p2p-port 9000

# Start with custom database
blockchain node --database /path/to/blockchain.db
```

#### Start API Server Only

Start just the REST API server without P2P networking:

```bash
blockchain api [OPTIONS]
```

**Options:**
- `--port <PORT>` - API server port (default: 8080)
- `--database <PATH>` - Database file path (default: blockchain.db)

**Examples:**
```bash
# Start API server on default port
blockchain api

# Start on custom port
blockchain api --port 3000
```

### Interactive Mode

Start the interactive CLI interface:

```bash
blockchain interactive
```

Or simply:

```bash
blockchain
```

The interactive mode provides a menu-driven interface for:
- Creating and managing wallets
- Mining blocks
- Creating transactions
- Viewing blockchain information
- Deploying smart contracts

### Wallet Management

#### Create Wallet

Create a new wallet with generated cryptographic keys:

```bash
blockchain create-wallet <NAME>
```

**Example:**
```bash
blockchain create-wallet "Alice's Wallet"
```

**Output:**
```
✅ Wallet created successfully!
Name: Alice's Wallet
Address: a1b2c3d4e5f6...
Public Key: 1234567890abcdef...
```

#### List Wallets

Display all stored wallets:

```bash
blockchain list-wallets
```

**Output:**
```
💳 Wallets:
  Alice's Wallet - a1b2c3d4e5f6... (created: 2023-12-01 10:30:00)
  Bob's Wallet - b2c3d4e5f6a1... (created: 2023-12-01 11:15:00)
```

#### Check Balance

Check the balance of a specific address:

```bash
blockchain balance <ADDRESS>
```

**Example:**
```bash
blockchain balance a1b2c3d4e5f6...
```

**Output:**
```
💰 Balance for a1b2c3d4e5f6...: 150.5
```

### Transaction Management

#### Create Transaction

Create a new transaction:

```bash
blockchain transaction <FROM> <TO> <AMOUNT> [DATA]
```

**Parameters:**
- `FROM` - Sender address
- `TO` - Recipient address
- `AMOUNT` - Transaction amount
- `DATA` - Optional transaction data/memo

**Examples:**
```bash
# Simple transaction
blockchain transaction alice bob 100.0

# Transaction with memo
blockchain transaction alice bob 50.0 "Payment for services"
```

**Output:**
```
✅ Transaction created: tx_123456789
💡 Add this transaction to a block using the mining feature
```

### Mining Operations

#### Mine Block

Mine a new block:

```bash
blockchain mine <MINER_ADDRESS> [OPTIONS]
```

**Options:**
- `--difficulty <LEVEL>` - Mining difficulty (default: 4)

**Examples:**
```bash
# Mine with default difficulty
blockchain mine miner_address_123

# Mine with custom difficulty
blockchain mine miner_address_123 --difficulty 6
```

**Note:** Mining in CLI mode is simplified. For full mining capabilities, use the node mode or API.

### Blockchain Information

#### Show Blockchain Info

Display general blockchain information:

```bash
blockchain info
```

**Output:**
```
🔗 Blockchain Information:
  Length: 42 blocks
  Valid: true
  Latest block: #41
  Latest hash: 000abc123def456...
```

#### Validate Blockchain

Validate the entire blockchain integrity:

```bash
blockchain validate
```

**Output:**
```
✅ Blockchain is valid!
```

### Smart Contract Operations

#### Deploy Contract

Deploy a smart contract:

```bash
blockchain deploy-contract <NAME> <WASM_FILE> <OWNER>
```

**Parameters:**
- `NAME` - Contract name
- `WASM_FILE` - Path to WASM bytecode file
- `OWNER` - Owner address

**Example:**
```bash
blockchain deploy-contract "MyContract" contract.wasm alice_address
```

**Note:** Contract deployment in CLI mode is simplified. Use the API for full functionality.

#### Call Contract

Call a smart contract function:

```bash
blockchain call-contract <CONTRACT_ID> <FUNCTION> <CALLER>
```

**Parameters:**
- `CONTRACT_ID` - Contract identifier
- `FUNCTION` - Function name to call
- `CALLER` - Caller address

**Example:**
```bash
blockchain call-contract contract_123 "transfer" alice_address
```

## Configuration

### Using Configuration Files

Set the configuration file path using the environment variable:

```bash
export BLOCKCHAIN_CONFIG=config/production.toml
blockchain node
```

Or specify for a single command:

```bash
BLOCKCHAIN_CONFIG=config/development.toml blockchain node
```

### Available Configurations

- `config/default.toml` - Default settings
- `config/development.toml` - Development environment
- `config/production.toml` - Production environment
- `config/miner.toml` - Mining node optimized

### Configuration Precedence

1. Environment variables
2. Configuration file
3. Command-line arguments
4. Default values

## Environment Variables

Key environment variables:

- `BLOCKCHAIN_CONFIG` - Path to configuration file
- `BLOCKCHAIN_DATA_DIR` - Data directory path
- `BLOCKCHAIN_LOG_LEVEL` - Logging level (debug, info, warn, error)
- `RUST_LOG` - Rust logging configuration

**Examples:**

```bash
# Set log level
export RUST_LOG=debug
blockchain node

# Set data directory
export BLOCKCHAIN_DATA_DIR=/var/lib/blockchain
blockchain node

# Multiple environment variables
RUST_LOG=info BLOCKCHAIN_CONFIG=config/production.toml blockchain node
```

## Exit Codes

The CLI uses standard exit codes:

- `0` - Success
- `1` - General error
- `2` - Invalid arguments
- `3` - Configuration error
- `4` - Database error
- `5` - Network error

## Examples and Use Cases

### Development Workflow

1. **Start development node:**
```bash
BLOCKCHAIN_CONFIG=config/development.toml blockchain node --api-port 8081
```

2. **Create test wallets:**
```bash
blockchain create-wallet "Alice"
blockchain create-wallet "Bob"
```

3. **Create test transactions:**
```bash
blockchain transaction alice bob 100.0 "test payment"
```

4. **Check balances:**
```bash
blockchain balance alice
blockchain balance bob
```

### Production Deployment

1. **Start production node:**
```bash
BLOCKCHAIN_CONFIG=config/production.toml blockchain node
```

2. **Monitor in separate terminal:**
```bash
# Check node health
curl http://localhost:8080/api/v1/blockchain/info

# Monitor logs
tail -f logs/blockchain.log
```

### Mining Setup

1. **Create miner wallet:**
```bash
blockchain create-wallet "Miner"
```

2. **Start mining node:**
```bash
BLOCKCHAIN_CONFIG=config/miner.toml blockchain node --p2p-port 9001
```

3. **Monitor mining:**
```bash
curl http://localhost:8080/api/v1/mining/stats
```

### Testing and Validation

1. **Run blockchain validation:**
```bash
blockchain validate
```

2. **Check specific block:**
```bash
curl http://localhost:8080/api/v1/blocks/5
```

3. **Verify transaction:**
```bash
curl http://localhost:8080/api/v1/transactions/tx_123
```

## Troubleshooting

### Common Issues

#### Permission Denied
```bash
# Make binary executable
chmod +x target/release/blockchain

# Or run via cargo
cargo run -- node
```

#### Port Already in Use
```bash
# Check what's using the port
lsof -i :8080

# Use different port
blockchain node --api-port 8081
```

#### Database Locked
```bash
# Ensure no other instances are running
ps aux | grep blockchain

# Remove lock file if safe
rm blockchain.db-wal blockchain.db-shm
```

#### Configuration Not Found
```bash
# Check config file exists
ls -la config/

# Use absolute path
BLOCKCHAIN_CONFIG=/full/path/to/config.toml blockchain node
```

### Debug Mode

Run with debug logging:

```bash
RUST_LOG=debug blockchain node
```

Run with trace logging (very verbose):

```bash
RUST_LOG=trace blockchain node
```

### Log Locations

Default log locations:
- Development: `./logs/`
- Production: `/var/log/blockchain/`
- Container: `/var/lib/blockchain/logs/`

### Health Checks

Check if the node is running properly:

```bash
# API health
curl -f http://localhost:8080/api/v1/blockchain/info

# Mining status
curl http://localhost:8080/api/v1/mining/stats

# Network status
curl http://localhost:8080/api/v1/network/stats
```

## Integration Examples

### Shell Scripts

```bash
#!/bin/bash
# start-blockchain.sh

export RUST_LOG=info
export BLOCKCHAIN_CONFIG=config/production.toml

blockchain node --api-port 8080 --p2p-port 9000
```

### Systemd Service

```ini
[Unit]
Description=Blockchain Node
After=network.target

[Service]
Type=simple
User=blockchain
WorkingDirectory=/opt/blockchain
Environment=RUST_LOG=info
Environment=BLOCKCHAIN_CONFIG=/opt/blockchain/config/production.toml
ExecStart=/opt/blockchain/target/release/blockchain node
Restart=always
RestartSec=5

[Install]
WantedBy=multi-user.target
```

### Docker Integration

```bash
# Build container
docker build -t blockchain:latest .

# Run container
docker run -d \\
  --name blockchain-node \\
  -p 8080:8080 \\
  -p 9000:9000 \\
  -e RUST_LOG=info \\
  blockchain:latest
```

## API Integration

The CLI works seamlessly with the REST API. When running in node mode, you can use both CLI commands and API calls:

```bash
# Start node
blockchain node &

# Use API
curl http://localhost:8080/api/v1/blockchain/info

# Use CLI (in another terminal)
blockchain info
```

## Security Considerations

- **Private Keys**: CLI stores private keys locally. Ensure proper file permissions.
- **Network Security**: Use firewalls to protect P2P ports in production.
- **Configuration**: Store sensitive configuration securely.
- **Logging**: Be careful not to log sensitive information.

## Performance Tips

- Use SSD storage for better database performance
- Increase system file descriptor limits for many connections
- Use production configuration for optimal settings
- Monitor resource usage with system tools

## Getting Help

```bash
# General help
blockchain --help

# Command-specific help
blockchain node --help
blockchain api --help

# Version information
blockchain --version
```

For more detailed information, see:
- [API Documentation](API.md)
- [Configuration Guide](../README.md#configuration)
- [Deployment Guide](../README.md#deployment)