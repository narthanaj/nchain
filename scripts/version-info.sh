#!/bin/bash

# Version Information Script
# Shows detailed version information for the blockchain project

set -e

# Colors for output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${BLUE}🔗 Blockchain Node - Version Information${NC}"
echo "=========================================="

# Project version
if [ -f Cargo.toml ]; then
    PROJECT_VERSION=$(grep '^version = ' Cargo.toml | head -1 | cut -d'"' -f2)
    echo -e "${GREEN}Project Version:${NC} v${PROJECT_VERSION}"
else
    echo -e "${YELLOW}Warning: Cargo.toml not found${NC}"
fi

# Rust version
if command -v rustc &> /dev/null; then
    RUST_VERSION=$(rustc --version)
    echo -e "${GREEN}Rust Version:${NC} ${RUST_VERSION}"

    # Check if version meets requirements
    RUST_VERSION_NUM=$(rustc --version | cut -d' ' -f2)
    MIN_VERSION="1.90.0"

    if [[ "$(printf '%s\n' "$MIN_VERSION" "$RUST_VERSION_NUM" | sort -V | head -n1)" == "$MIN_VERSION" ]]; then
        echo -e "${GREEN}✅ Rust version meets requirements (>= $MIN_VERSION)${NC}"
    else
        echo -e "${YELLOW}⚠️  Rust version is below recommended ($MIN_VERSION)${NC}"
    fi
else
    echo -e "${YELLOW}⚠️  Rust not found${NC}"
fi

# Cargo version
if command -v cargo &> /dev/null; then
    CARGO_VERSION=$(cargo --version)
    echo -e "${GREEN}Cargo Version:${NC} ${CARGO_VERSION}"
else
    echo -e "${YELLOW}⚠️  Cargo not found${NC}"
fi

# Edition
if [ -f Cargo.toml ]; then
    EDITION=$(grep '^edition = ' Cargo.toml | head -1 | cut -d'"' -f2)
    echo -e "${GREEN}Rust Edition:${NC} ${EDITION}"
fi

# Git info (if available)
if command -v git &> /dev/null && [ -d .git ]; then
    GIT_COMMIT=$(git rev-parse --short HEAD 2>/dev/null || echo "unknown")
    GIT_BRANCH=$(git branch --show-current 2>/dev/null || echo "unknown")
    GIT_STATUS=$(git status --porcelain 2>/dev/null | wc -l | tr -d ' ')

    echo -e "${GREEN}Git Commit:${NC} ${GIT_COMMIT}"
    echo -e "${GREEN}Git Branch:${NC} ${GIT_BRANCH}"

    if [ "$GIT_STATUS" -eq 0 ]; then
        echo -e "${GREEN}Git Status:${NC} Clean"
    else
        echo -e "${YELLOW}Git Status:${NC} ${GIT_STATUS} modified files"
    fi
fi

# Build information
echo ""
echo -e "${BLUE}📦 Build Information${NC}"
echo "===================="

# Target platform
if command -v rustc &> /dev/null; then
    TARGET=$(rustc -vV | grep "host:" | cut -d' ' -f2)
    echo -e "${GREEN}Target Platform:${NC} ${TARGET}"
fi

# Profile information
echo -e "${GREEN}Available Profiles:${NC} dev, release, test, bench"

# Dependencies count
if [ -f Cargo.toml ]; then
    DEPS_COUNT=$(grep -c '^[a-zA-Z].*=' Cargo.toml | head -1 || echo "0")
    echo -e "${GREEN}Dependencies:${NC} ~${DEPS_COUNT} packages"
fi

# Binary size (if exists)
if [ -f target/release/blockchain ]; then
    BINARY_SIZE=$(ls -lh target/release/blockchain | awk '{print $5}')
    echo -e "${GREEN}Release Binary Size:${NC} ${BINARY_SIZE}"
fi

# System information
echo ""
echo -e "${BLUE}🖥️  System Information${NC}"
echo "====================="

# OS information
if command -v uname &> /dev/null; then
    OS_NAME=$(uname -s)
    OS_RELEASE=$(uname -r)
    ARCHITECTURE=$(uname -m)

    echo -e "${GREEN}Operating System:${NC} ${OS_NAME} ${OS_RELEASE}"
    echo -e "${GREEN}Architecture:${NC} ${ARCHITECTURE}"
fi

# CPU information (Linux/macOS)
if [ "$(uname)" == "Darwin" ]; then
    # macOS
    CPU_INFO=$(sysctl -n machdep.cpu.brand_string 2>/dev/null || echo "Unknown")
    CPU_CORES=$(sysctl -n hw.ncpu 2>/dev/null || echo "Unknown")
    echo -e "${GREEN}CPU:${NC} ${CPU_INFO}"
    echo -e "${GREEN}CPU Cores:${NC} ${CPU_CORES}"
elif [ "$(uname)" == "Linux" ]; then
    # Linux
    if [ -f /proc/cpuinfo ]; then
        CPU_INFO=$(grep "model name" /proc/cpuinfo | head -1 | cut -d':' -f2 | sed 's/^ *//')
        CPU_CORES=$(nproc 2>/dev/null || echo "Unknown")
        echo -e "${GREEN}CPU:${NC} ${CPU_INFO}"
        echo -e "${GREEN}CPU Cores:${NC} ${CPU_CORES}"
    fi
fi

# Memory information
if [ "$(uname)" == "Darwin" ]; then
    # macOS
    MEMORY_GB=$(expr $(sysctl -n hw.memsize) / 1024 / 1024 / 1024 2>/dev/null || echo "Unknown")
    echo -e "${GREEN}Memory:${NC} ${MEMORY_GB}GB"
elif [ "$(uname)" == "Linux" ]; then
    # Linux
    if [ -f /proc/meminfo ]; then
        MEMORY_KB=$(grep "MemTotal" /proc/meminfo | awk '{print $2}')
        MEMORY_GB=$(expr $MEMORY_KB / 1024 / 1024 2>/dev/null || echo "Unknown")
        echo -e "${GREEN}Memory:${NC} ${MEMORY_GB}GB"
    fi
fi

# Environment variables
echo ""
echo -e "${BLUE}🌍 Environment${NC}"
echo "=============="

# Rust-related environment variables
[ -n "$RUST_LOG" ] && echo -e "${GREEN}RUST_LOG:${NC} $RUST_LOG"
[ -n "$RUSTFLAGS" ] && echo -e "${GREEN}RUSTFLAGS:${NC} $RUSTFLAGS"
[ -n "$CARGO_TARGET_DIR" ] && echo -e "${GREEN}CARGO_TARGET_DIR:${NC} $CARGO_TARGET_DIR"

# Blockchain-specific environment variables
[ -n "$BLOCKCHAIN_CONFIG" ] && echo -e "${GREEN}BLOCKCHAIN_CONFIG:${NC} $BLOCKCHAIN_CONFIG"
[ -n "$BLOCKCHAIN_DATA_DIR" ] && echo -e "${GREEN}BLOCKCHAIN_DATA_DIR:${NC} $BLOCKCHAIN_DATA_DIR"

# Build timestamp
echo ""
echo -e "${GREEN}Report Generated:${NC} $(date)"

echo ""
echo -e "${BLUE}🚀 Ready to build blockchain! 🚀${NC}"