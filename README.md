# Polymarket P&L Substreams

A comprehensive Substreams package that captures all Polymarket trading data and P&L calculations, perfectly matching the Dune Analytics query structure.

## 🎯 Features

- **Complete Data Capture**: All 13 data sources from the Dune query
- **Real-time Streaming**: Live data from Polygon mainnet
- **Perfect Dune Match**: 100% data structure compatibility
- **Comprehensive P&L**: Trading, liquidity, and reward calculations
- **Market Metadata**: Question data and market information
- **Price Discovery**: Real-time price tracking from order fills

## 🚀 Quick Start

### 1. Test Endpoint
```bash
python3 examples/test_scripts/test_proper_endpoint.py
```

### 2. Stream 7 Days of Data
```bash
python3 scripts/stream_7days_proper.py
```

### 3. View Dashboard
The script will create a comprehensive Dune-style dashboard with:
- Top 25 traders leaderboard
- P&L analytics
- Market activity summary
- Complete data export

## 📊 Data Sources Captured

1. **Market Creation** - CTF Exchange & Neg Risk markets
2. **Trading Activity** - ERC1155 transfers & order fills
3. **USDC Transfers** - Trading-related USDC movements
4. **Liquidity Rewards** - UMA & USDC Merkle Distributor claims
5. **AMM Markets** - Fixed Product Market Maker creation
6. **Price Data** - Real-time price discovery
7. **Question Metadata** - Market questions and details
8. **Batch Transfers** - Multi-token transfers
9. **Additional Rewards** - USDC distributor claims
10. **Transaction Hashes** - Real blockchain transaction IDs
11. **Block Timestamps** - Precise timing data
12. **User Balances** - Share values and USDC positions
13. **P&L Calculations** - Comprehensive profit/loss tracking

## 🏗️ Architecture

```
src/
├── lib.rs          # Main processing logic
├── abi.rs          # ABI decoding functions
└── ...

proto/
└── contract.proto  # Protobuf message definitions

scripts/
├── stream_7days_proper.py    # Main 7-day streaming
├── stream_7days_simple.py    # Simplified streaming
└── stream_recent_data.py     # Recent data streaming

examples/
├── old_dashboards/           # Dashboard implementations
└── test_scripts/            # Testing utilities
```

## 🔧 Configuration

The package uses the official Polygon endpoint:
- **Endpoint**: `polygon.streamingfast.io:443`
- **Network**: Polygon Mainnet
- **Authentication**: Via environment variables

## 📈 Output Format

Perfect match with Dune query structure:
```json
{
  "userPnls": [...],      // User P&L data
  "marketData": [...],    // Market information
  "tokenTransfers": [...], // ERC1155 transfers
  "orderFills": [...],    // Trading orders
  "rewardClaims": [...]   // Liquidity rewards
}
```

## 🎯 Dune Query Compatibility

This Substreams package provides 100% data structure compatibility with:
- [Dune Query #3366316](https://dune.com/queries/3366316)
- All 13 data sources captured
- Identical field names and types
- Real-time streaming capability

## 🚀 Next Steps

1. **Stream Data**: Run the 7-day streaming script
2. **Build Dashboard**: Use the generated data for web interface
3. **Real-time Updates**: Set up continuous streaming
4. **Scale**: Deploy for production use

## 📚 Documentation

- [Substreams Documentation](https://docs.substreams.dev/)
- [Polygon Endpoints](https://docs.substreams.dev/reference-material/chains-and-endpoints)
- [Dune Query Reference](https://dune.com/queries/3366316)

---

**Ready to build your Polymarket leaderboard!** 🎉