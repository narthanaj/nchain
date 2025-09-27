#!/bin/bash

# Blockchain Node Setup Script
# This script sets up the blockchain node environment

set -e

echo "🔗 Setting up Blockchain Node..."

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function to print colored output
print_status() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Check if Rust is installed
if ! command -v cargo &> /dev/null; then
    print_error "Rust is not installed. Please install Rust from https://rustup.rs/"
    exit 1
fi

print_success "Rust is installed: $(rustc --version)"

# Check Rust version
RUST_VERSION=$(rustc --version | cut -d' ' -f2)
MIN_VERSION="1.90.0"

if [[ "$(printf '%s\n' "$MIN_VERSION" "$RUST_VERSION" | sort -V | head -n1)" != "$MIN_VERSION" ]]; then
    print_error "Rust version $RUST_VERSION is too old. Minimum required: $MIN_VERSION"
    print_status "Update Rust with: rustup update"
    exit 1
fi

# Check if SQLite is available
if ! command -v sqlite3 &> /dev/null; then
    print_warning "SQLite3 is not installed. Installing..."
    if [[ "$OSTYPE" == "linux-gnu"* ]]; then
        sudo apt-get update && sudo apt-get install -y sqlite3 libsqlite3-dev
    elif [[ "$OSTYPE" == "darwin"* ]]; then
        brew install sqlite
    else
        print_error "Please install SQLite3 manually"
        exit 1
    fi
fi

print_success "SQLite is available: $(sqlite3 --version)"

# Create necessary directories
print_status "Creating directory structure..."
mkdir -p data logs config docs scripts

# Copy example configuration if it doesn't exist
if [ ! -f .env ]; then
    if [ -f .env.example ]; then
        cp .env.example .env
        print_success "Created .env from .env.example"
    fi
fi

# Set permissions
chmod +x scripts/*.sh

# Build the project
print_status "Building blockchain project..."
if cargo build --release; then
    print_success "Build completed successfully"
else
    print_error "Build failed"
    exit 1
fi

# Run tests
print_status "Running tests..."
if cargo test; then
    print_success "All tests passed"
else
    print_warning "Some tests failed"
fi

# Initialize database (if needed)
print_status "Checking database..."
if [ ! -f data/blockchain.db ]; then
    print_status "Initializing database..."
    # The database will be created automatically on first run
    print_success "Database will be initialized on first run"
fi

# Check if ports are available
check_port() {
    local port=$1
    local service=$2

    if lsof -Pi :$port -sTCP:LISTEN -t >/dev/null 2>&1; then
        print_warning "Port $port is already in use (needed for $service)"
        return 1
    else
        print_success "Port $port is available for $service"
        return 0
    fi
}

print_status "Checking port availability..."
check_port 8080 "API server"
check_port 9000 "P2P networking"

# Create systemd service file (optional)
if [[ "$OSTYPE" == "linux-gnu"* ]] && [ -d "/etc/systemd/system" ]; then
    if [ "$EUID" -eq 0 ]; then
        print_status "Creating systemd service file..."
        cat > /etc/systemd/system/blockchain.service << EOF
[Unit]
Description=Blockchain Node
After=network.target

[Service]
Type=simple
User=blockchain
WorkingDirectory=$(pwd)
Environment=RUST_LOG=info
ExecStart=$(pwd)/target/release/blockchain node
Restart=always
RestartSec=5

[Install]
WantedBy=multi-user.target
EOF
        systemctl daemon-reload
        print_success "Systemd service created. Enable with: sudo systemctl enable blockchain"
    else
        print_status "Run as root to create systemd service"
    fi
fi

# Print summary
echo ""
echo "=========================================="
print_success "Blockchain Node setup completed!"
echo "=========================================="
echo ""
echo "Next steps:"
echo "1. Review configuration in config/ directory"
echo "2. Modify .env file if needed"
echo "3. Start the node:"
echo "   ./target/release/blockchain node"
echo ""
echo "Or start with cargo:"
echo "   cargo run -- node"
echo ""
echo "API will be available at: http://localhost:8080"
echo "P2P networking on port: 9000"
echo ""
echo "For help:"
echo "   ./target/release/blockchain --help"
echo ""
echo "Documentation:"
echo "   - README.md"
echo "   - docs/API.md"
echo "   - docs/CLI.md"
echo ""
print_success "Happy blockchain building! 🚀"