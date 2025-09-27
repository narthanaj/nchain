#!/bin/bash

# Development Start Script
# Starts the blockchain node in development mode

set -e

# Colors for output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}🚀 Starting Blockchain Node in Development Mode...${NC}"

# Set development environment
export RUST_LOG=debug
export BLOCKCHAIN_CONFIG=config/development.toml

# Create development data directory
mkdir -p dev-data logs

# Check if development config exists
if [ ! -f config/development.toml ]; then
    echo "Development config not found, using default config"
    export BLOCKCHAIN_CONFIG=config/default.toml
fi

# Start the node
echo -e "${GREEN}Starting node with development configuration...${NC}"
echo "API Server: http://localhost:8081"
echo "P2P Port: 9001"
echo "Database: dev-data/blockchain.db"
echo "Log Level: DEBUG"
echo ""
echo "Press Ctrl+C to stop"
echo ""

cargo run -- node --api-port 8081 --p2p-port 9001 --database dev-data/blockchain.db