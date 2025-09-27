# 🔗 Rust Blockchain Prototype

A high-performance, Solana-inspired blockchain implementation built in Rust, featuring simplified Proof-of-History (PoH) consensus and a beautiful CLI interface.

## ✨ Features

- **🏗️ Modern Architecture**: Modular design with proper error handling and type safety
- **⚡ Proof-of-History**: Simplified implementation of Solana's innovative consensus mechanism
- **🔒 Cryptographic Security**: SHA-256 hashing for block integrity and chain validation
- **🎨 Beautiful CLI**: Interactive command-line interface with colored output
- **🧪 Comprehensive Testing**: Full test suite ensuring reliability
- **📊 Balance Tracking**: Built-in wallet balance calculation
- **✅ Chain Validation**: Real-time blockchain integrity verification

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

- Rust 1.70+ (install from [rustup.rs](https://rustup.rs/))

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

### Usage

#### Interactive Mode (Recommended)

Start the interactive CLI:
```bash
cargo run
```

This will open an interactive menu where you can:
- ✅ Add new blocks with transactions
- 📊 View the entire blockchain
- 🔍 Validate chain integrity
- 💰 Check account balances
- 📈 View blockchain statistics

#### Command Line Interface

You can also use direct commands:

```bash
# Add a new block with transactions
cargo run -- add-block "alice:bob:100.0:payment" "bob:charlie:50.0:transfer"

# Show the blockchain
cargo run -- show

# Validate the blockchain
cargo run -- validate

# Check balance for an address
cargo run -- balance alice
```

#### Transaction Format

Transactions use the format: `from:to:amount:data`
- `from`: Sender address
- `to`: Recipient address
- `amount`: Transfer amount (positive number)
- `data`: Optional transaction data

Example: `"alice:bob:100.0:monthly rent"`

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

## 🔮 Future Enhancements

Potential improvements for this prototype:

- **Networking**: Peer-to-peer blockchain network
- **Persistence**: Database storage for blockchain data
- **Smart Contracts**: Simple contract execution engine
- **Web Interface**: REST API and web dashboard
- **Mining**: Configurable difficulty and rewards
- **Digital Signatures**: Transaction signing with public/private keys

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