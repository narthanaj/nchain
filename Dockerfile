# Multi-stage build for optimal image size and security
FROM rust:1.90.0-slim as builder

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    libsqlite3-dev \
    && rm -rf /var/lib/apt/lists/*

# Create app directory
WORKDIR /app

# Copy dependency files
COPY Cargo.toml Cargo.lock ./

# Create a dummy main.rs to build dependencies first (for better caching)
RUN mkdir src && echo "fn main() {}" > src/main.rs

# Build dependencies
RUN cargo build --release && rm src/main.rs

# Copy source code
COPY src/ ./src/
COPY config/ ./config/
COPY migrations/ ./migrations/

# Build the application
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    libssl3 \
    libsqlite3-0 \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Create blockchain user and group
RUN groupadd -r blockchain && useradd -r -g blockchain blockchain

# Create necessary directories
RUN mkdir -p /var/lib/blockchain /var/log/blockchain /etc/blockchain && \
    chown -R blockchain:blockchain /var/lib/blockchain /var/log/blockchain /etc/blockchain

# Copy binary from builder stage
COPY --from=builder /app/target/release/blockchain /usr/local/bin/blockchain

# Copy configuration files
COPY --from=builder /app/config/ /etc/blockchain/config/
COPY --from=builder /app/migrations/ /etc/blockchain/migrations/

# Copy production configuration as default
COPY --from=builder /app/config/production.toml /etc/blockchain/blockchain.toml

# Set permissions
RUN chown -R blockchain:blockchain /etc/blockchain && \
    chmod +x /usr/local/bin/blockchain

# Switch to blockchain user
USER blockchain

# Set working directory
WORKDIR /var/lib/blockchain

# Create data directory
RUN mkdir -p data logs

# Expose ports
EXPOSE 8080 9000

# Health check
HEALTHCHECK --interval=30s --timeout=10s --start-period=60s --retries=3 \
    CMD curl -f http://localhost:8080/api/v1/blockchain/info || exit 1

# Environment variables
ENV BLOCKCHAIN_CONFIG=/etc/blockchain/blockchain.toml
ENV BLOCKCHAIN_DATA_DIR=/var/lib/blockchain/data
ENV BLOCKCHAIN_LOG_DIR=/var/lib/blockchain/logs
ENV RUST_LOG=info

# Default command
CMD ["blockchain", "node", "--api-port", "8080", "--p2p-port", "9000"]