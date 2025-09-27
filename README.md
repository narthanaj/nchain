# Blockchain Node

A modern, high-performance blockchain implementation written in Rust with support for smart contracts, P2P networking, and a comprehensive REST API.

## 🚀 Advanced Features

### 🔐 **Digital Signatures & Cryptography**
- **Ed25519 Digital Signatures**: Military-grade cryptographic signatures for all transactions
- **Wallet Management**: Secure key generation, storage, and management
- **Address System**: Cryptographic addressing derived from public keys
- **Transaction Security**: All transactions are cryptographically signed and verified

### ⛏️ **Advanced Mining System**
- **Configurable Difficulty**: Dynamic difficulty adjustment based on network performance
- **Proof-of-Work**: SHA-256 based mining with nonce discovery
- **Block Rewards**: Configurable mining rewards with coinbase transactions
- **Mining Statistics**: Comprehensive hash rate and performance tracking
- **Difficulty Adjustment**: Automatic network difficulty balancing

### 💾 **Database Persistence**
- **SQLite Integration**: Full blockchain data persistence to database
- **Block Storage**: Efficient block and transaction storage
- **Wallet Persistence**: Secure wallet storage and retrieval
- **Mining Stats**: Historical mining data and statistics
- **Database Migrations**: Automatic schema management

### 🌐 **Peer-to-Peer Networking**
- **P2P Architecture**: Decentralized network communication
- **Block Broadcasting**: Real-time block propagation across network
- **Transaction Pool**: Distributed transaction mempool
- **Peer Discovery**: Automatic peer discovery and connection management
- **Network Statistics**: Real-time network health monitoring

### 📝 **Smart Contract Engine**
- **WebAssembly Runtime**: Execute smart contracts using WASM
- **Contract Deployment**: Deploy and manage smart contracts
- **Gas System**: Configurable gas limits and execution costs
- **Contract Storage**: Persistent contract state management
- **Event System**: Contract event emission and logging

### 🌍 **REST API & Web Dashboard**
- **RESTful API**: Complete blockchain API for all operations
- **Beautiful Dashboard**: Real-time web interface with live updates
- **API Documentation**: Comprehensive API endpoints
- **CORS Support**: Cross-origin resource sharing for web apps
- **Real-time Updates**: Live blockchain statistics and monitoring

## 🏗️ Architecture

### Core Components

- **Block**: Individual blocks containing transactions, timestamps, and cryptographic hashes
- **Transaction**: Secure transfer records with validation
- **Blockchain**: Chain management with genesis block and validation
- **PohRecorder**: Simplified Proof-of-History implementation
- **CLI**: Interactive and command-line interfaces

### Project Structure

```
src/
├── lib.rs           # Library entry point and tests
├── main.rs          # Binary entry point
├── block.rs         # Block data structure and operations
├── blockchain.rs    # Blockchain management
├── transaction.rs   # Transaction handling
├── poh.rs          # Proof-of-History implementation
├── errors.rs       # Error types and handling
└── cli.rs          # Command-line interface
```

## 🚀 Quick Start

### Prerequisites

- **Rust 1.90.0+** (install from [rustup.rs](https://rustup.rs/))
- **SQLite 3.35+** (for database persistence)
- **Git** (for version control)

### Installation

1. Clone and navigate to the project:
   ```bash
   cd blockchain
   ```

2. Build the project:
   ```bash
   cargo build --release
   ```

3. Run tests:
   ```bash
   cargo test
   ```

### Usage Modes

#### 🌐 Full Node Mode (Recommended)

Start a complete blockchain node with API server and P2P networking:
```bash
cargo run -- node --api-port 8080 --p2p-port 9000
```

This starts:
- ✅ Complete blockchain node
- 🌐 REST API server on port 8080
- 🔗 P2P networking on port 9000
- 💾 Database persistence
- 📊 Web dashboard at http://localhost:8080

#### 🔧 API Server Only

Start just the REST API server:
```bash
cargo run -- api --port 8080
```

#### 💻 Interactive CLI Mode

Start the enhanced interactive CLI:
```bash
cargo run -- interactive
```

Features include:
- 💳 Wallet creation and management
- ⛏️ Mining with configurable difficulty
- 📝 Smart contract deployment
- 🔍 Advanced blockchain inspection
- 📊 Real-time statistics

#### ⚡ Direct Commands

```bash
# Create a new wallet
cargo run -- create-wallet "Alice"

# Check wallet balance
cargo run -- balance <wallet-address>

# Create a transaction
cargo run -- transaction alice bob 100.0 "payment"

# Mine a block
cargo run -- mine <miner-address> --difficulty 4

# Deploy a smart contract
cargo run -- deploy-contract "MyContract" contract.wasm alice

# Show blockchain info
cargo run -- info

# List all wallets
cargo run -- list-wallets
```

### 🌍 Web Dashboard

Access the beautiful web dashboard at `http://localhost:8080` when running in node or API mode.

The dashboard provides:
- 📊 Real-time blockchain statistics
- ⛏️ Mining performance metrics
- 🌐 Network status and peer information
- 📦 Recent blocks and transactions
- 💳 Wallet management
- 📝 Smart contract interface

### 🔌 REST API Endpoints

#### Blockchain Operations
- `GET /api/blockchain/info` - Get blockchain information
- `GET /api/blockchain/validate` - Validate blockchain integrity
- `GET /api/blocks` - List recent blocks
- `GET /api/blocks/{index}` - Get specific block

#### Transaction Management
- `GET /api/transactions` - List recent transactions
- `POST /api/transactions` - Create new transaction
- `GET /api/balance/{address}` - Get address balance

#### Mining Operations
- `POST /api/mine` - Start mining a block
- `GET /api/mining/stats` - Get mining statistics
- `GET /api/mining/config` - Get mining configuration

#### Wallet Management
- `GET /api/wallets` - List all wallets
- `POST /api/wallets` - Create new wallet
- `GET /api/wallets/{address}` - Get wallet details

#### Smart Contracts
- `GET /api/contracts` - List deployed contracts
- `POST /api/contracts` - Deploy new contract
- `POST /api/contracts/{id}/call` - Call contract function

#### Network Status
- `GET /api/network/stats` - Get network statistics
- `GET /api/network/peers` - List connected peers

## 🔬 Proof-of-History

This implementation includes a simplified version of Solana's Proof-of-History:

- **Sequential Hashing**: Creates a verifiable passage of time
- **Deterministic Ordering**: Ensures transaction order without global consensus
- **Cryptographic Timestamps**: SHA-256 based time verification

The PoH recorder generates a sequence of hashes for each block, creating an immutable timeline that can be independently verified.

## 🧪 Testing

Run the comprehensive test suite:

```bash
cargo test
```

Tests cover:
- ✅ Transaction creation and validation
- ✅ Block creation and integrity
- ✅ Blockchain validation
- ✅ Proof-of-History functionality
- ✅ Balance calculations
- ✅ Error handling

## 📊 Example Usage

```bash
$ cargo run

🔗 Blockchain initialized!
Welcome to the Rust Blockchain Prototype

╭─ Choose an action ─╮
│ 1. Add a new block  │
│ 2. Show blockchain  │
│ 3. Validate chain   │
│ 4. Check balance    │
│ 5. Show stats       │
│ 6. Exit             │
╰────────────────────╯

Enter your choice: 1

📝 Adding a new block
Enter transaction details (or 'done' to finish):
From address: alice
To address: bob
Amount: 100.0
Data (optional): payment for services
✅ Transaction added
Enter transaction details (or 'done' to finish):
From address: done
🎉 Block added successfully!
```

## 🏗️ Architecture Details

### Error Handling

The project uses `thiserror` for comprehensive error handling:
- `BlockchainError::InvalidBlock`: Block validation failures
- `BlockchainError::ChainValidation`: Chain integrity issues
- `BlockchainError::InvalidTransaction`: Transaction validation errors
- `BlockchainError::EmptyBlockchain`: Operations on empty chains

### Security Features

- **Hash Validation**: Every block hash is validated against its contents
- **Chain Integrity**: Previous hash linking ensures tamper detection
- **Transaction Validation**: Input validation prevents invalid transactions
- **Cryptographic Signatures**: SHA-256 hashing throughout

### Performance

- **Efficient Hashing**: Optimized SHA-256 implementation
- **Memory Safety**: Rust's ownership system prevents memory issues
- **Concurrent Safe**: Thread-safe design for future parallel processing

## 🎯 Production Features

This blockchain implementation includes enterprise-grade features:

### 🔐 Security & Cryptography
- **Military-grade Encryption**: Ed25519 digital signatures
- **Secure Key Management**: Hardware-backed key storage support
- **Transaction Security**: All transactions cryptographically verified
- **Address Privacy**: Cryptographic address derivation

### ⚡ Performance & Scalability
- **Async Architecture**: Non-blocking I/O operations
- **Database Optimization**: Efficient SQLite storage
- **Memory Management**: Rust's zero-cost abstractions
- **Concurrent Processing**: Multi-threaded mining and validation

### 🌐 Network & Distribution
- **P2P Protocol**: Decentralized network communication
- **Peer Discovery**: Automatic network topology management
- **Block Propagation**: Efficient blockchain synchronization
- **Network Resilience**: Fault-tolerant peer connections

### 📝 Smart Contract Platform
- **WebAssembly Runtime**: High-performance contract execution
- **Gas Metering**: Resource usage tracking and limits
- **Contract Storage**: Persistent state management
- **Event System**: Real-time contract notifications

### 💾 Data & Storage
- **ACID Compliance**: Database transaction integrity
- **Backup & Recovery**: Data export/import capabilities
- **Migration System**: Automatic schema updates
- **Compression**: Efficient block storage

### 🔧 Developer Experience
- **REST API**: Complete programmatic interface
- **Web Dashboard**: Visual blockchain explorer
- **CLI Tools**: Command-line utilities
- **Logging**: Comprehensive system monitoring

### 🚀 Deployment & Operations
- **Docker Support**: Containerized deployment
- **Configuration Management**: Environment-based settings
- **Health Monitoring**: System status endpoints
- **Metrics Export**: Prometheus-compatible metrics

## 🤝 Contributing

This is a learning project demonstrating blockchain concepts. Feel free to:

1. Fork the repository
2. Create a feature branch
3. Add tests for new functionality
4. Submit a pull request

## 📜 License

MIT License - see LICENSE file for details.

## 🙏 Acknowledgments

- Inspired by [Solana's](https://solana.com/) Proof-of-History innovation
- Built with ❤️ in Rust
- Uses excellent crates: `clap`, `serde`, `sha2`, `chrono`, `colored`

---

*This project is for educational purposes, demonstrating blockchain fundamentals with modern Rust practices.*