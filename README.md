# Polymarket P&L Substreams 📊

**Real-time Profit & Loss tracking for Polymarket prediction markets on Polygon**

[![Substreams](https://img.shields.io/badge/Substreams-v0.1.0-blue)](https://substreams.dev)
[![Network](https://img.shields.io/badge/Network-Polygon-purple)](https://polygon.technology)
[![License](https://img.shields.io/badge/License-MIT-green)](LICENSE)

## 🎯 Overview

This Substreams package provides comprehensive real-time P&L tracking for Polymarket prediction markets, monitoring all core contracts and calculating user positions, profits, and losses as they happen.

## 🏗️ Architecture

### Core Contracts Tracked

1. **Conditional Tokens Framework (CTF)** - `0x4D97DCd97eC945f40cF65F87097ACe5EA0476045`
   - Position splits, merges, and redemptions
   - Condition preparation and resolution
   - Token transfers

2. **CTF Exchange (Orderbook)** - `0x4bfb41d5b3570defd03c39a9a4d8de6bd8b8982e`
   - Order fills and matches
   - Trading fees
   - Order cancellations

3. **USDC Collateral Token** - `0x2791Bca1f2de4661ED88A30C99A7a9449Aa84174`
   - Transfer events
   - Approval events

## 📊 Modules

### 1. `map_ctf_events`
Tracks all Conditional Tokens Framework events:
- `ConditionPreparation` - New market conditions
- `ConditionResolution` - Market outcomes and payouts
- `PositionSplit` - Position splitting events
- `PositionMerge` - Position merging events
- `PositionRedeem` - Position redemption events
- `TransferSingle` & `TransferBatch` - Token transfers

### 2. `map_ctf_exchange_events`
Tracks CTF Exchange orderbook events:
- `OrderFilled` - Completed trades
- `OrdersMatched` - Order matching
- `OrderCancelled` - Cancelled orders
- `FeeCharged` - Trading fees
- `TokenRegistered` - New token registrations

### 3. `map_usdc_events`
Tracks USDC collateral token events:
- `Transfer` - USDC transfers
- `Approval` - USDC approvals

### 4. `map_user_positions`
Real-time user position tracking:
- Current token holdings per user
- Buy/sell transaction history
- Average prices and realized P&L
- Position updates and changes

### 5. `map_pnl_data`
Comprehensive P&L calculations:
- User-level P&L metrics
- Market-level P&L data
- Global platform statistics
- Volume and trade counts

## 🚀 Quick Start

### Prerequisites
- [Substreams CLI](https://docs.substreams.dev/getting-started/installation)
- Authentication token from [The Graph Market](https://market.thegraph.com)

### Installation

```bash
# Clone the repository
git clone https://github.com/PaulieB14/Polymarket-PnL-Substreams.git
cd Polymarket-PnL-Substreams

# Build the package
substreams build

# Authenticate
export SUBSTREAMS_API_TOKEN="your_jwt_token_here"

# Run live streaming
substreams gui
```

### Usage Examples

#### Track CTF Events
```bash
substreams run substreams.yaml map_ctf_events --start-block 4023686
```

#### Monitor User Positions
```bash
substreams run substreams.yaml map_user_positions --start-block 4023686
```

#### Get P&L Data
```bash
substreams run substreams.yaml map_pnl_data --start-block 4023686
```

## 📈 Data Schema

### User Position
```protobuf
message UserPosition {
    string user_address = 1;
    string token_id = 2;
    string condition_id = 3;
    string outcome_index = 4;
    string amount_held = 5;
    string average_price = 6;
    string total_bought = 7;
    string total_sold = 8;
    string realized_pnl = 9;
    string unrealized_pnl = 10;
    google.protobuf.Timestamp first_seen = 11;
    google.protobuf.Timestamp last_updated = 12;
}
```

### User P&L
```protobuf
message UserPnL {
    string user_address = 1;
    string total_realized_pnl = 2;
    string total_unrealized_pnl = 3;
    string total_volume = 4;
    string total_trades = 5;
    string winning_trades = 6;
    string losing_trades = 7;
    string win_rate = 8;
    google.protobuf.Timestamp last_activity = 9;
}
```

### Market P&L
```protobuf
message MarketPnL {
    string condition_id = 1;
    string question_id = 2;
    string total_volume = 3;
    string total_trades = 4;
    string total_fees = 5;
    string winning_outcome = 6;
    string resolution_price = 7;
    google.protobuf.Timestamp created_at = 8;
    google.protobuf.Timestamp resolved_at = 9;
}
```

## 🔧 Configuration

### Network
- **Chain**: Polygon
- **Start Block**: 4023686 (CTF contract deployment)
- **Exchange Start Block**: 33605403 (CTF Exchange deployment)

### Performance
- **Parallel Processing**: Yes
- **Block Filtering**: Optimized for specific contract addresses
- **Memory Efficient**: Streaming data processing

## 📊 Use Cases

### 1. Real-time P&L Dashboard
Build live dashboards showing user profits/losses as they trade.

### 2. Risk Management
Monitor user positions and exposure in real-time.

### 3. Market Analysis
Track market volumes, fees, and trading patterns.

### 4. Portfolio Tracking
Individual user portfolio performance and history.

### 5. Compliance & Reporting
Automated P&L reporting for tax and regulatory purposes.

## 🛠️ Development

### Building
```bash
substreams build
```

### Testing
```bash
# Test with live data
substreams run substreams.yaml map_pnl_data --start-block 4023686

# Test specific module
substreams run substreams.yaml map_user_positions --start-block 4023686
```

### Publishing
```bash
# Login to registry
substreams registry login

# Publish package
substreams registry publish
```

## 📚 Related Projects

- [Polymarket Trading Substreams](https://substreams.dev/packages/polymarket/v0.1.0) - General trading data
- [Polymarket P&L Subgraph](https://github.com/Polymarket/polymarket-subgraph/tree/main/pnl-subgraph) - Historical P&L data
- [Polymarket Documentation](https://docs.polymarket.com) - Official Polymarket docs

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests if applicable
5. Submit a pull request

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🆘 Support

- **Documentation**: [Substreams Docs](https://docs.substreams.dev)
- **Community**: [Substreams Discord](https://discord.gg/substreams)
- **Issues**: [GitHub Issues](https://github.com/PaulieB14/Polymarket-PnL-Substreams/issues)

## 🎉 Acknowledgments

- [StreamingFast](https://streamingfast.io) for Substreams technology
- [Polymarket](https://polymarket.com) for the prediction market platform
- [The Graph](https://thegraph.com) for indexing infrastructure

---

**Built with ❤️ for the Polymarket community**