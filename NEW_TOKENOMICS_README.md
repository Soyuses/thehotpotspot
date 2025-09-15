# 🚀 New Tokenomics Model - The Hot Pot Spot

## Overview

This document describes the new tokenomics model implemented for The Hot Pot Spot project, featuring Security Tokens (ST) and Utility Tokens (UT) with streaming integration.

## 🎯 Key Features

### Security Tokens (ST)
- **Purpose**: Digital shares with dividend rights
- **Minting**: 1 ST per 1 GEL spent (configurable)
- **KYC Required**: Yes, for transfers and claims
- **Transfer Restrictions**: Yes, until KYC verification
- **Dividend Eligibility**: Yes, for verified holders

### Utility Tokens (UT)
- **Purpose**: DAO voting power and conversion participation
- **Non-transferable**: Soulbound tokens (SBT)
- **Earning Methods**:
  - Streaming: 10 UT per minute (max 2 hours)
  - Comments: 5 UT per comment
  - Shares: 20 UT per share
  - Likes: 2 UT per like
  - Views: 1 UT per view
- **Daily Limit**: 1000 UT per user per day

## 🏗️ Architecture

### Core Components

1. **NewTokenomicsManager** (`src/new_tokenomics.rs`)
   - Manages ST and UT token balances
   - Handles conversion rounds
   - Tracks sales and events

2. **NewRelayerService** (`src/new_relayer_service.rs`)
   - Processes POS sales
   - Mints ST tokens
   - Manages check claiming

3. **StreamCollector** (`src/stream_collector.rs`)
   - Tracks streaming sessions
   - Awards UT tokens for engagement
   - Manages user interactions

4. **NewDatabaseManager** (`src/new_database.rs`)
   - PostgreSQL integration
   - User management
   - Event tracking

### Database Schema

The new tokenomics model uses the following main tables:

- `users` - User accounts with KYC status
- `sales` - Sale records with ST token allocation
- `st_mintings` - ST token minting history
- `ut_events` - UT token earning events
- `streaming_sessions` - Active streaming sessions
- `conversion_rounds` - UT to ST conversion rounds
- `conversion_allocations` - Individual conversion allocations

## 🔄 Business Logic

### Customer Journey

1. **Purchase**: Customer buys food and receives check with QR code
2. **Choice**: Customer can either:
   - Discard the check (ST tokens remain in reserve)
   - Scan QR code to claim ST tokens
3. **Claiming**: Customer downloads mobile app and claims ST tokens
4. **KYC**: Customer completes KYC verification for transfers
5. **Streaming**: Customer watches streams and earns UT tokens
6. **Conversion**: UT holders participate in quarterly conversion rounds

### Conversion Mechanism

- **Frequency**: Quarterly (configurable)
- **Pool Size**: 50% of reserved ST tokens
- **Distribution**: Proportional to UT token holdings
- **KYC Requirement**: Yes, for receiving converted tokens

## 🛠️ Configuration

### TokenomicsConfig
```rust
pub struct TokenomicsConfig {
    pub security_token: SecurityTokenConfig,
    pub utility_token: UtilityTokenConfig,
    pub conversion: ConversionConfig,
    pub governance: GovernanceConfig,
}
```

### RelayerConfig
```rust
pub struct RelayerConfig {
    pub min_amount_gel: f64,        // 1.0
    pub max_amount_gel: f64,        // 1000.0
    pub st_per_gel: u128,          // 100 (1 ST per 1 GEL)
    pub kyc_required: bool,        // true
}
```

### StreamCollectorConfig
```rust
pub struct StreamCollectorConfig {
    pub ut_per_minute: u128,       // 10
    pub ut_per_comment: u128,      // 5
    pub ut_per_share: u128,        // 20
    pub ut_per_like: u128,         // 2
    pub ut_per_view: u128,         // 1
    pub max_ut_per_day: u128,      // 1000
    pub min_streaming_minutes: u32, // 5
    pub max_streaming_minutes: u32, // 120
}
```

## 🚀 Getting Started

### Prerequisites

- Rust 1.70+
- PostgreSQL 13+
- Tokio runtime

### Installation

1. Clone the repository:
```bash
git clone https://github.com/Soyuses/thehotpotspot.git
cd thehotpotspot
```

2. Install dependencies:
```bash
cargo build
```

3. Set up database:
```bash
# Create database
createdb thehotpotspot

# Run migrations
psql thehotpotspot < migrations/001_new_tokenomics_tables.sql
```

4. Run the demo:
```bash
cargo run --example new_tokenomics_demo
```

## 📊 Demo Example

The `examples/new_tokenomics_demo.rs` demonstrates:

1. **POS Sale Processing**: Creating and processing a sale
2. **ST Token Minting**: Minting ST tokens for purchases
3. **Check Claiming**: Customer claiming ST tokens from check
4. **Streaming Activities**: Starting/ending streaming sessions
5. **UT Token Earning**: Earning UT tokens through engagement
6. **Statistics**: Viewing system statistics
7. **Conversion Rounds**: Triggering conversion rounds

### Sample Output

```
🚀 The Hot Pot Spot - New Tokenomics Demo
==========================================
📊 Configuration:
  - ST per GEL: 100
  - UT per minute: 10
  - UT per comment: 5
  - UT per share: 20
  - Max UT per day: 1000

🛒 Simulating POS Sale...
✅ Sale processed successfully!
  - Sale ID: sale_001
  - Check Address: 0x...
  - Activation Code: 123456
  - ST Units: 2500

📱 Simulating Customer Check Claim...
✅ ST tokens claimed successfully!
  - Mint ID: CLAIM_...
  - Units: 2500
  - To Address: 0x...

🎥 Simulating Streaming Activities...
✅ Streaming session started!
✅ Streaming session completed!
  - Duration: 30 minutes
  - UT Earned: 300

💬 Simulating User Interactions...
✅ Comment recorded! UT earned: 5
✅ Share recorded! UT earned: 20
✅ Like recorded! UT earned: 2
✅ View recorded! UT earned: 1

📊 Getting Statistics...
📈 Relayer Statistics:
  - Total Sales: 1
  - Successful Sales: 1
  - Total ST Minted: 2500
  - Total Amount (GEL): 25.00

📈 Stream Collector Statistics:
  - Total Sessions: 1
  - Completed Sessions: 1
  - Total UT Awarded: 328

🔄 Simulating Conversion Round...
✅ Conversion round triggered!
  - Round ID: round_...
  - Total Pool: 1250
  - Total UT Snapshot: 328
```

## 🔒 Security Features

### KYC Integration
- Required for ST token transfers
- Required for conversion round participation
- Status tracking and validation

### Token Restrictions
- ST tokens: Transfer-restricted until KYC
- UT tokens: Non-transferable (SBT)
- Daily limits on UT earning

### Audit Trail
- Complete transaction history
- Event logging
- Statistics tracking

## 📈 Monitoring

### Statistics Available
- Total ST/UT holders
- Total sales processed
- Total tokens minted/awarded
- Conversion round statistics
- User engagement metrics

### Database Views
- `user_balance_summary` - User balance overview
- `conversion_round_stats` - Conversion statistics
- `governance_proposal_results` - Voting results

## 🧪 Testing

Run the test suite:
```bash
cargo test
```

Run specific tests:
```bash
cargo test new_tokenomics
cargo test relayer_service
cargo test stream_collector
```

## 📝 API Reference

### NewTokenomicsManager
- `record_sale()` - Record a sale and mint ST tokens
- `claim_st_tokens()` - Claim ST tokens from check
- `add_ut_event()` - Add UT earning event
- `trigger_conversion_round()` - Start conversion round
- `get_statistics()` - Get system statistics

### NewRelayerService
- `process_sale()` - Process POS sale request
- `claim_st_tokens()` - Claim ST tokens for user
- `get_stats()` - Get relayer statistics

### StreamCollector
- `start_streaming_session()` - Start streaming session
- `end_streaming_session()` - End streaming session
- `record_comment()` - Record user comment
- `record_share()` - Record content share
- `record_like()` - Record content like
- `record_view()` - Record content view

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests
5. Submit a pull request

## 📄 License

This project is licensed under the MIT License - see the LICENSE file for details.

## 🆘 Support

For questions or support, please:
- Open an issue on GitHub
- Contact the development team
- Check the documentation

---

**Note**: This is a demonstration implementation. For production use, additional security measures, testing, and compliance checks should be implemented.
