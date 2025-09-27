# Blockchain Node - Setup and Configuration Guide

This guide provides a comprehensive overview of the blockchain node setup, configuration, and deployment options.

## 📁 Project Structure

```
blockchain/
├── src/                           # Source code
│   ├── lib.rs                     # Library entry point
│   ├── main.rs                    # Binary entry point
│   ├── block.rs                   # Block implementation
│   ├── blockchain.rs              # Blockchain logic
│   ├── transaction.rs             # Transaction handling
│   ├── poh.rs                     # Proof of History
│   ├── errors.rs                  # Error types
│   ├── cli.rs                     # CLI interface
│   ├── crypto.rs                  # Cryptography
│   ├── mining.rs                  # Mining functionality
│   ├── storage.rs                 # Database operations
│   ├── network.rs                 # P2P networking
│   ├── contracts.rs               # Smart contracts
│   ├── api.rs                     # REST API
│   └── config.rs                  # Configuration system
├── config/                        # Configuration files
│   ├── default.toml               # Default configuration
│   ├── development.toml           # Development settings
│   ├── production.toml            # Production settings
│   └── miner.toml                 # Mining node settings
├── migrations/                    # Database migrations
│   ├── 001_initial_schema.sql     # Initial database schema
│   └── 002_add_performance_indexes.sql # Performance optimizations
├── docs/                          # Documentation
│   ├── API.md                     # API documentation
│   └── CLI.md                     # CLI documentation
├── scripts/                       # Utility scripts
│   ├── setup.sh                   # Setup script
│   └── start-dev.sh               # Development start script
├── Dockerfile                     # Docker container definition
├── docker-compose.yml             # Docker Compose setup
├── .dockerignore                  # Docker ignore file
├── .env.example                   # Environment variables example
├── README.md                      # Main documentation
└── Cargo.toml                     # Rust dependencies
```

## 🚀 Quick Start

### 1. Prerequisites

- **Rust 1.90.0+**: Install from [rustup.rs](https://rustup.rs/)
- **SQLite 3.35+**: For database storage
- **Git**: For cloning and version control

### 2. Setup

Run the automated setup script:

```bash
./scripts/setup.sh
```

Or manual setup:

```bash
# Build the project
cargo build --release

# Create directories
mkdir -p data logs

# Copy environment file
cp .env.example .env
```

### 3. Start the Node

#### Development Mode
```bash
./scripts/start-dev.sh
```
- API: http://localhost:8081
- P2P: port 9001
- Database: dev-data/blockchain.db
- Logging: DEBUG level

#### Production Mode
```bash
BLOCKCHAIN_CONFIG=config/production.toml cargo run -- node
```
- API: http://localhost:8080
- P2P: port 9000
- Database: data/blockchain.db
- Logging: INFO level

## ⚙️ Configuration

### Configuration Files

The blockchain node uses TOML configuration files located in the `config/` directory:

| File | Purpose | Use Case |
|------|---------|----------|
| `default.toml` | Default settings | Base configuration |
| `development.toml` | Development | Local development |
| `production.toml` | Production | Live deployment |
| `miner.toml` | Mining nodes | Dedicated miners |

### Configuration Sections

#### Node Configuration
```toml
[node]
name = "blockchain-node"          # Node identifier
data_dir = "./data"               # Data directory
genesis_block_reward = 50.0       # Genesis block reward
```

#### Database Configuration
```toml
[database]
url = "sqlite:./data/blockchain.db"  # Database URL
max_connections = 10              # Connection pool size
connection_timeout_secs = 30      # Connection timeout
enable_migrations = true          # Auto-run migrations
```

#### API Configuration
```toml
[api]
enabled = true                    # Enable API server
bind_address = "127.0.0.1"        # Bind address
port = 8080                       # API port
cors_enabled = true               # CORS support
rate_limit_requests_per_minute = 100  # Rate limiting
```

#### Mining Configuration
```toml
[mining]
enabled = false                   # Enable mining
difficulty = 4                    # Mining difficulty
block_reward = 12.5               # Block reward
target_block_time_secs = 600      # Target block time (10 minutes)
```

#### Network Configuration
```toml
[network]
enabled = false                   # Enable P2P networking
listen_port = 9000                # P2P port
max_peers = 50                    # Maximum peer connections
bootstrap_peers = []              # Initial peer list
```

#### Smart Contracts Configuration
```toml
[contracts]
enabled = true                    # Enable smart contracts
max_memory_mb = 16                # Maximum contract memory
execution_timeout_secs = 30       # Execution timeout
max_gas_limit = 1000000           # Maximum gas per transaction
```

### Environment Variables

Key environment variables:

```bash
# Configuration file
BLOCKCHAIN_CONFIG=config/production.toml

# Data directory
BLOCKCHAIN_DATA_DIR=/var/lib/blockchain

# Logging
RUST_LOG=info

# Database
DATABASE_URL=sqlite:./data/blockchain.db
```

## 🗄️ Database

### Schema

The blockchain uses SQLite with the following main tables:

- **blocks** - Blockchain blocks
- **transactions** - All transactions
- **wallets** - Wallet information
- **smart_contracts** - Deployed contracts
- **contract_states** - Contract state storage
- **mining_stats** - Mining statistics
- **network_peers** - P2P network peers
- **system_metadata** - System configuration

### Migrations

Database migrations are automatically applied when `database.enable_migrations = true`.

Manual migration:
```bash
sqlite3 data/blockchain.db < migrations/001_initial_schema.sql
sqlite3 data/blockchain.db < migrations/002_add_performance_indexes.sql
```

### Backup

```bash
# Backup database
sqlite3 data/blockchain.db ".backup backup.db"

# Restore database
cp backup.db data/blockchain.db
```

## 🌐 API Endpoints

### Base URL: `http://localhost:8080/api/v1`

| Category | Endpoint | Method | Description |
|----------|----------|--------|-------------|
| **Blockchain** | `/blockchain/info` | GET | Blockchain information |
| | `/blockchain/validate` | GET | Validate blockchain |
| **Blocks** | `/blocks` | GET | List blocks |
| | `/blocks/{index}` | GET | Get specific block |
| **Transactions** | `/transactions` | GET | List transactions |
| | `/transactions/{id}` | GET | Get specific transaction |
| | `/transactions` | POST | Create transaction |
| **Wallets** | `/wallets` | GET | List wallets |
| | `/wallets` | POST | Create wallet |
| | `/wallets/{address}` | GET | Get wallet details |
| **Mining** | `/mining/mine` | POST | Start mining |
| | `/mining/stats` | GET | Mining statistics |
| **Contracts** | `/contracts` | GET | List contracts |
| | `/contracts` | POST | Deploy contract |
| | `/contracts/{id}/call` | POST | Call contract |
| **Network** | `/network/stats` | GET | Network statistics |

See [docs/API.md](docs/API.md) for detailed API documentation.

## 💻 CLI Commands

### Node Operations
```bash
blockchain node                           # Start full node
blockchain api                            # Start API only
blockchain interactive                     # Interactive mode
```

### Wallet Management
```bash
blockchain create-wallet "Alice"          # Create wallet
blockchain list-wallets                   # List wallets
blockchain balance <address>              # Check balance
```

### Transactions
```bash
blockchain transaction alice bob 100.0    # Create transaction
```

### Mining
```bash
blockchain mine <miner-address>           # Mine block
```

### Information
```bash
blockchain info                           # Blockchain info
blockchain validate                       # Validate chain
```

See [docs/CLI.md](docs/CLI.md) for detailed CLI documentation.

## 🐳 Docker Deployment

### Build and Run

```bash
# Build image
docker build -t blockchain:latest .

# Run single container
docker run -d \\
  --name blockchain-node \\
  -p 8080:8080 \\
  -p 9000:9000 \\
  blockchain:latest

# Run with Docker Compose
docker-compose up -d
```

### Docker Compose Services

- **blockchain-node** - Main blockchain node
- **blockchain-miner** - Dedicated mining node
- **blockchain-dev** - Development node
- **nginx** - Reverse proxy (production profile)
- **prometheus** - Monitoring (monitoring profile)
- **grafana** - Dashboard (monitoring profile)

### Docker Profiles

```bash
# Development
docker-compose --profile development up -d

# Production with monitoring
docker-compose --profile production --profile monitoring up -d
```

## 📊 Monitoring and Logging

### Log Levels

- **trace** - Very detailed logs
- **debug** - Debug information
- **info** - General information (default)
- **warn** - Warning messages
- **error** - Error messages only

### Log Configuration

```toml
[logging]
level = "info"                    # Log level
format = "json"                   # json or pretty
output = "console"                # console, file, or both
```

### Health Checks

```bash
# API health
curl http://localhost:8080/api/v1/blockchain/info

# Mining status
curl http://localhost:8080/api/v1/mining/stats

# Network status
curl http://localhost:8080/api/v1/network/stats
```

## 🔒 Security

### Production Security Checklist

- [ ] Use HTTPS for API endpoints
- [ ] Configure firewall rules
- [ ] Secure private key storage
- [ ] Enable proper logging
- [ ] Use strong passwords/keys
- [ ] Regular security updates
- [ ] Monitor for suspicious activity
- [ ] Backup encryption

### Network Security

```bash
# Allow API port
sudo ufw allow 8080/tcp

# Allow P2P port
sudo ufw allow 9000/tcp

# Deny all other incoming
sudo ufw default deny incoming
```

## 🔧 Development

### Building

```bash
# Development build
cargo build

# Release build
cargo build --release

# Run tests
cargo test

# Run with logging
RUST_LOG=debug cargo run
```

### Development Workflow

1. Start development node: `./scripts/start-dev.sh`
2. Make changes to source code
3. Rebuild: `cargo build`
4. Test: `cargo test`
5. Restart node if needed

### Testing

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_blockchain_creation

# Run with output
cargo test -- --nocapture
```

## 📋 Deployment Checklist

### Production Deployment

1. **Pre-deployment**
   - [ ] Review configuration files
   - [ ] Set up monitoring
   - [ ] Configure backups
   - [ ] Security audit
   - [ ] Performance testing

2. **Deployment**
   - [ ] Deploy with production config
   - [ ] Verify database migrations
   - [ ] Check API endpoints
   - [ ] Verify P2P connectivity
   - [ ] Test mining (if enabled)

3. **Post-deployment**
   - [ ] Monitor logs
   - [ ] Check resource usage
   - [ ] Verify backup systems
   - [ ] Performance monitoring
   - [ ] Security monitoring

### Scaling Considerations

- **Horizontal scaling**: Deploy multiple nodes
- **Load balancing**: Use nginx or similar
- **Database optimization**: Monitor query performance
- **Caching**: Implement Redis if needed
- **Monitoring**: Use Prometheus + Grafana

## 🆘 Troubleshooting

### Common Issues

1. **Build failures**: Check Rust version and dependencies
2. **Port conflicts**: Change ports in configuration
3. **Database issues**: Check permissions and migrations
4. **Network problems**: Verify firewall settings
5. **Performance issues**: Check resource usage

### Debug Commands

```bash
# Check running processes
ps aux | grep blockchain

# Check port usage
lsof -i :8080

# Check database
sqlite3 data/blockchain.db ".tables"

# View logs
tail -f logs/blockchain.log
```

### Getting Help

- 📖 Documentation: [README.md](README.md)
- 🔌 API Guide: [docs/API.md](docs/API.md)
- 💻 CLI Guide: [docs/CLI.md](docs/CLI.md)
- 🐛 Issues: Check GitHub issues

## 🎯 Next Steps

After setup, consider:

1. **Development**: Implement custom features
2. **Integration**: Connect to existing systems
3. **Scaling**: Deploy multiple nodes
4. **Monitoring**: Set up comprehensive monitoring
5. **Security**: Implement production security measures
6. **Performance**: Optimize for your use case

Happy blockchain building! 🚀