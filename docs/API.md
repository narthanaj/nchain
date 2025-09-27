# Blockchain API Documentation

This document provides comprehensive documentation for the Blockchain Node REST API.

## Base URL

```
http://localhost:8080/api/v1
```

## Authentication

Currently, the API does not require authentication. In a production environment, you would typically implement JWT or API key authentication.

## Response Format

All API responses follow a consistent format:

```json
{
  "success": true,
  "data": { ... },
  "error": null
}
```

For errors:

```json
{
  "success": false,
  "data": null,
  "error": "Error message description"
}
```

## Rate Limiting

The API implements rate limiting based on the configuration:
- Default: 100 requests per minute per IP
- Configurable via `api.rate_limit_requests_per_minute`

## Endpoints

### Blockchain Operations

#### Get Blockchain Information

```http
GET /api/v1/blockchain/info
```

Returns general information about the blockchain.

**Response:**
```json
{
  "success": true,
  "data": {
    "length": 42,
    "latest_hash": "000abc123...",
    "latest_block_index": 41,
    "total_transactions": 156,
    "is_valid": true,
    "difficulty": 4
  }
}
```

#### Validate Blockchain

```http
GET /api/v1/blockchain/validate
```

Validates the entire blockchain integrity.

**Response:**
```json
{
  "success": true,
  "data": "Blockchain is valid"
}
```

### Block Operations

#### List Blocks

```http
GET /api/v1/blocks?limit=10&offset=0
```

Returns a paginated list of blocks, newest first.

**Query Parameters:**
- `limit` (optional): Number of blocks to return (default: 10, max: 100)
- `offset` (optional): Number of blocks to skip (default: 0)

**Response:**
```json
{
  "success": true,
  "data": [
    {
      "index": 41,
      "timestamp": "2023-12-01T10:30:00Z",
      "transactions": [...],
      "previous_hash": "000def456...",
      "hash": "000abc123...",
      "poh_hash": "poh789...",
      "nonce": 142857,
      "difficulty": 4,
      "miner": "miner_address_123"
    }
  ]
}
```

#### Get Specific Block

```http
GET /api/v1/blocks/{index}
```

Returns a specific block by its index.

**Path Parameters:**
- `index`: Block index (0 for genesis block)

**Response:**
```json
{
  "success": true,
  "data": {
    "index": 5,
    "timestamp": "2023-12-01T10:30:00Z",
    "transactions": [...],
    "previous_hash": "000def456...",
    "hash": "000abc123...",
    "poh_hash": "poh789...",
    "nonce": 142857,
    "difficulty": 4,
    "miner": "miner_address_123"
  }
}
```

### Transaction Operations

#### List Transactions

```http
GET /api/v1/transactions?limit=20
```

Returns a paginated list of transactions, newest first.

**Query Parameters:**
- `limit` (optional): Number of transactions to return (default: 20, max: 100)

**Response:**
```json
{
  "success": true,
  "data": [
    {
      "id": "tx_123456",
      "from": "alice_address",
      "to": "bob_address",
      "amount": 100.0,
      "data": "Payment for services",
      "timestamp": "2023-12-01T10:30:00Z",
      "signature": "signature_data...",
      "from_public_key": "public_key_data..."
    }
  ]
}
```

#### Get Specific Transaction

```http
GET /api/v1/transactions/{id}
```

Returns a specific transaction by its ID.

**Path Parameters:**
- `id`: Transaction ID

**Response:**
```json
{
  "success": true,
  "data": {
    "id": "tx_123456",
    "from": "alice_address",
    "to": "bob_address",
    "amount": 100.0,
    "data": "Payment for services",
    "timestamp": "2023-12-01T10:30:00Z",
    "signature": "signature_data...",
    "from_public_key": "public_key_data..."
  }
}
```

#### Create Transaction

```http
POST /api/v1/transactions
```

Creates a new transaction.

**Request Body:**
```json
{
  "from": "alice_address",
  "to": "bob_address",
  "amount": 100.0,
  "data": "Payment description",
  "private_key": "optional_hex_private_key"
}
```

**Parameters:**
- `from`: Sender address (required)
- `to`: Recipient address (required)
- `amount`: Transaction amount (required, >= 0)
- `data`: Optional transaction data/memo
- `private_key`: Optional hex-encoded private key for signing

**Response:**
```json
{
  "success": true,
  "data": {
    "id": "tx_789012",
    "from": "alice_address",
    "to": "bob_address",
    "amount": 100.0,
    "data": "Payment description",
    "timestamp": "2023-12-01T10:35:00Z",
    "signature": "signature_data...",
    "from_public_key": "public_key_data..."
  }
}
```

### Balance Operations

#### Get Address Balance

```http
GET /api/v1/addresses/{address}/balance
```

Returns the current balance for an address.

**Path Parameters:**
- `address`: The address to check balance for

**Response:**
```json
{
  "success": true,
  "data": 250.5
}
```

### Wallet Operations

#### List Wallets

```http
GET /api/v1/wallets
```

Returns a list of all wallets stored in the system.

**Response:**
```json
{
  "success": true,
  "data": [
    {
      "name": "Alice's Wallet",
      "address": "alice_address_123",
      "created_at": "2023-12-01T09:00:00Z"
    }
  ]
}
```

#### Create Wallet

```http
POST /api/v1/wallets
```

Creates a new wallet with generated keys.

**Request Body:**
```json
{
  "name": "My New Wallet"
}
```

**Response:**
```json
{
  "success": true,
  "data": {
    "name": "My New Wallet",
    "address": "new_wallet_address_456",
    "public_key": "public_key_hex...",
    "private_key": "private_key_hex..."
  }
}
```

#### Get Wallet Details

```http
GET /api/v1/wallets/{address}
```

Returns details for a specific wallet.

**Path Parameters:**
- `address`: Wallet address

**Response:**
```json
{
  "success": true,
  "data": {
    "name": "Alice's Wallet",
    "address": "alice_address_123",
    "public_key": "public_key_hex...",
    "balance": 150.0
  }
}
```

### Mining Operations

#### Start Mining

```http
POST /api/v1/mining/mine
```

Starts mining a new block.

**Request Body:**
```json
{
  "miner_address": "miner_address_123",
  "difficulty": 4
}
```

**Response:**
```json
{
  "success": true,
  "data": "Mining started"
}
```

#### Get Mining Statistics

```http
GET /api/v1/mining/stats
```

Returns current mining statistics.

**Response:**
```json
{
  "success": true,
  "data": {
    "blocks_mined": 15,
    "total_hash_rate": 1500000,
    "current_difficulty": 4,
    "last_block_time": "2023-12-01T10:30:00Z",
    "average_block_time": 598.5
  }
}
```

#### Get Mining Configuration

```http
GET /api/v1/mining/config
```

Returns current mining configuration.

**Response:**
```json
{
  "success": true,
  "data": {
    "difficulty": 4,
    "block_reward": 12.5,
    "max_block_time": 300,
    "difficulty_adjustment_interval": 2016,
    "target_block_time": 600
  }
}
```

#### Update Mining Configuration

```http
PUT /api/v1/mining/config
```

Updates mining configuration.

**Request Body:**
```json
{
  "difficulty": 5,
  "block_reward": 10.0,
  "max_block_time": 400
}
```

**Response:**
```json
{
  "success": true,
  "data": "Mining configuration updated"
}
```

### Smart Contract Operations

#### List Contracts

```http
GET /api/v1/contracts
```

Returns a list of deployed smart contracts.

**Response:**
```json
{
  "success": true,
  "data": [
    {
      "id": "contract_123",
      "name": "SimpleStorage",
      "owner": "owner_address",
      "gas_limit": 1000000,
      "created_at": "2023-12-01T09:00:00Z"
    }
  ]
}
```

#### Deploy Contract

```http
POST /api/v1/contracts
```

Deploys a new smart contract.

**Request Body:**
```json
{
  "name": "MyContract",
  "code": "base64_encoded_wasm_bytecode",
  "owner": "owner_address",
  "gas_limit": 1000000
}
```

**Response:**
```json
{
  "success": true,
  "data": {
    "id": "contract_456",
    "name": "MyContract",
    "owner": "owner_address",
    "gas_limit": 1000000,
    "deployed": true
  }
}
```

#### Get Contract Details

```http
GET /api/v1/contracts/{id}
```

Returns details for a specific contract.

**Path Parameters:**
- `id`: Contract ID

**Response:**
```json
{
  "success": true,
  "data": {
    "id": "contract_123",
    "name": "SimpleStorage",
    "owner": "owner_address",
    "gas_limit": 1000000,
    "created_at": "2023-12-01T09:00:00Z"
  }
}
```

#### Call Contract Function

```http
POST /api/v1/contracts/{id}/call
```

Calls a function on a deployed smart contract.

**Path Parameters:**
- `id`: Contract ID

**Request Body:**
```json
{
  "function_name": "store",
  "args": [42],
  "caller": "caller_address",
  "gas_limit": 100000,
  "value": 0.0
}
```

**Response:**
```json
{
  "success": true,
  "data": {
    "success": true,
    "return_value": 42,
    "gas_used": 1500,
    "logs": ["Storage updated"],
    "events": [],
    "error": null
  }
}
```

### Network Operations

#### Get Network Statistics

```http
GET /api/v1/network/stats
```

Returns current network statistics.

**Response:**
```json
{
  "success": true,
  "data": {
    "connected_peers": 8,
    "total_blocks_received": 150,
    "total_transactions_received": 500,
    "pending_transactions": 12,
    "last_sync": "2023-12-01T10:30:00Z"
  }
}
```

## Error Codes

The API uses standard HTTP status codes:

- `200` - Success
- `400` - Bad Request (invalid parameters)
- `404` - Not Found (resource doesn't exist)
- `429` - Too Many Requests (rate limited)
- `500` - Internal Server Error

## Examples

### Using curl

#### Create a Transaction
```bash
curl -X POST http://localhost:8080/api/v1/transactions \\
  -H "Content-Type: application/json" \\
  -d '{
    "from": "alice",
    "to": "bob",
    "amount": 50.0,
    "data": "Payment for goods"
  }'
```

#### Get Blockchain Info
```bash
curl http://localhost:8080/api/v1/blockchain/info
```

#### Create a Wallet
```bash
curl -X POST http://localhost:8080/api/v1/wallets \\
  -H "Content-Type: application/json" \\
  -d '{"name": "Test Wallet"}'
```

#### Get Address Balance
```bash
curl http://localhost:8080/api/v1/addresses/alice/balance
```

### Using JavaScript/Fetch

```javascript
// Get blockchain info
const response = await fetch('http://localhost:8080/api/v1/blockchain/info');
const data = await response.json();
console.log(data);

// Create transaction
const transaction = await fetch('http://localhost:8080/api/v1/transactions', {
  method: 'POST',
  headers: {
    'Content-Type': 'application/json',
  },
  body: JSON.stringify({
    from: 'alice',
    to: 'bob',
    amount: 25.0,
    data: 'Test payment'
  })
});
const result = await transaction.json();
```

### Using Python/Requests

```python
import requests

# Get blockchain info
response = requests.get('http://localhost:8080/api/v1/blockchain/info')
print(response.json())

# Create wallet
wallet_data = {
    'name': 'Python Wallet'
}
response = requests.post(
    'http://localhost:8080/api/v1/wallets',
    json=wallet_data
)
print(response.json())
```

## WebSocket API (Future)

The following WebSocket endpoints are planned for real-time updates:

- `ws://localhost:8080/ws/blocks` - Real-time block updates
- `ws://localhost:8080/ws/transactions` - Real-time transaction updates
- `ws://localhost:8080/ws/mining` - Real-time mining statistics

## API Versioning

The API uses URL versioning (`/api/v1/`). When breaking changes are made, a new version will be introduced while maintaining backward compatibility for existing versions.

## CORS

Cross-Origin Resource Sharing (CORS) is enabled by default and can be configured via the `api.cors_enabled` and `api.cors_origins` configuration options.

## Security Considerations

- Always use HTTPS in production
- Implement proper authentication for sensitive operations
- Validate all input parameters
- Use rate limiting to prevent abuse
- Never expose private keys in API responses
- Implement proper logging for audit trails