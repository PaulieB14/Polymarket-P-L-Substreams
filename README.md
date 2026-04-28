# Polymarket P&L Substreams

<p align="center">
  <img src="./polymarket-pnl-icon.png" alt="Polymarket P&L" width="120"/>
</p>

<p align="center">
  <strong>Real-time Profit & Loss tracking for Polymarket prediction markets on Polygon</strong>
</p>

<p align="center">
  <a href="https://substreams.dev/packages/polymarket-pnl/v1.2.0">
    <img src="https://img.shields.io/badge/substreams.dev-v1.2.0-blue" alt="Substreams"/>
  </a>
  <a href="https://polygon.technology/">
    <img src="https://img.shields.io/badge/network-Polygon-8247E5" alt="Polygon"/>
  </a>
  <a href="https://www.postgresql.org/">
    <img src="https://img.shields.io/badge/sink-PostgreSQL-336791" alt="PostgreSQL"/>
  </a>
  <a href="LICENSE">
    <img src="https://img.shields.io/badge/license-MIT-green" alt="License"/>
  </a>
</p>

---

## Overview

Comprehensive Substreams package for tracking Polymarket P&L with **SQL sink support** for persistent state accumulation. Tracks all trading activity from CTF Exchange and Neg Risk Exchange contracts across **both CLOB v1 and CLOB v2**.

---

## CLOB v2 ready (since v1.2.0)

Polymarket migrated to [CLOB v2](https://docs.polymarket.com/v2-migration) at the **2026-04-28 ~11:00 UTC** cutover. v2 ships fresh Exchange contracts at new addresses, a redesigned `OrderFilled` event, a different fee model, and a new collateral wrapper (pUSD). This package indexes both contract generations side-by-side so a single P&L pipeline spans the migration.

### What changed in CLOB v2

| Concept | CLOB v1 | CLOB v2 |
|---------|---------|---------|
| **Exchange contracts** | `0x4bfb…982e` (CTF) / `0xC5d5…f80a` (NegRisk) | `0xE111…996B` (CTF) / `0xe222…0F59` (NegRisk) — fresh deploys |
| **Order uniqueness** | `nonce` per maker | `timestamp` (ms) — nonces removed |
| **Order side** | Inferred from `makerAssetId == 0` | Explicit `side` enum on the order and event (`BUY=0`, `SELL=1`) |
| **OrderFilled event** | 8 fields, including `makerAssetId` + `takerAssetId` | 10 fields: single `tokenId` + `side` + new `builder` (bytes32) + `metadata` (bytes32) |
| **Fees** | Embedded in order (`feeRateBps`), maker + taker | Protocol-determined at match time, **taker only**, dynamic per market via `getClobMarketInfo()` |
| **Collateral (wallet)** | USDC.e directly | **pUSD** — a 1:1-backed ERC-20 wrapper. USDC.e converts via `CollateralOnramp.wrap()` |
| **Collateral (CTF level)** | USDC.e | USDC.e (unchanged — pUSD is purely wallet-facing) |
| **Builder attribution** | HMAC headers on API orders | Single `builderCode` (bytes32) on the order, surfaced as `builder` on the event |
| **EIP-712 domain version** | `"1"` | `"2"` for exchange signing (L1 API auth still `"1"`) |
| **Open orders at cutover** | — | All wiped during the maintenance window |

### How this package handles it

1. **Contract dispatch.** `map_order_fills` watches all four Exchange addresses (v1 + v2). When a log lands on a v2 contract, it routes to the v2 decoder; v1 contracts still use the original v1 decoder. Both produce the same internal `OrderFilledEvent` shape, so the rest of the pipeline (stores, P&L math, SQL sink) sees one homogeneous stream.
2. **v2 → v1 field mapping.** The v2 event emits one `tokenId` with an explicit `side`. We synthesize the legacy `maker_asset_id` / `taker_asset_id` pair from `(side, tokenId)` using the same convention as v1 (collateral-side gets `"0"`, the conditional-token side gets the token id). This keeps your existing `WHERE side = 'buy'` queries and the cost-basis store working with zero changes.
3. **New columns surfaced.** v2's `builder` and `metadata` are decoded but currently land only in the in-memory event (not yet in the SQL trades table — see [Migration to richer v2 schema](#migration-to-richer-v2-schema) below for an opt-in extension).
4. **Source-of-truth tagging.** The `exchange` column tags every row with which contract emitted it: `ctf` / `neg_risk` (v1) or `ctf_v2` / `neg_risk_v2`. So you can filter, partition, or audit by generation.
5. **Fee column semantics.** v2 fees are taker-only and protocol-determined; they appear in the same `fee` field but represent a single realized taker fee rather than maker+taker fees.
6. **No schema migration required.** Existing `trades`, `user_pnl`, `user_positions`, `markets`, `daily_stats` tables work unchanged — v2 fills slot in as additional rows with the new `exchange` values.

### What stays the same

- Same chain (Polygon), same Firehose source, same RPC.
- Same `map_order_fills` → stores → analytics → `db_out` DAG.
- Same SQL sink configuration, same leaderboard / whale views.
- Same start block for historical backfill (33,605,403); v2 fills activate naturally at the v2 deploy block (84,902,353).

### Migration to richer v2 schema

If you want to query v2-specific fields like `builder` (for builder attribution analytics) or `metadata`, you can add columns and surface them in `db_out`:

```sql
ALTER TABLE trades
  ADD COLUMN exchange_version VARCHAR(4) DEFAULT 'v1',
  ADD COLUMN builder VARCHAR(66),
  ADD COLUMN metadata VARCHAR(66);
CREATE INDEX idx_trades_builder ON trades(builder) WHERE builder IS NOT NULL;
```

Then add the matching `.set("builder", ...)` / `.set("metadata", ...)` calls in `db_out` and republish. This is opt-in — the package as shipped doesn't require it.

### Key Features

| Feature | Description |
|---------|-------------|
| **CLOB v1 + v2** | Decodes both Exchange generations into the same P&L pipeline |
| **Real P&L Tracking** | Realized & unrealized P&L with cost basis |
| **SQL Sink** | PostgreSQL/Clickhouse for persistent state |
| **Trader Analytics** | Volume, win rate, max drawdown |
| **Market Stats** | Price, volume, trade counts per market |
| **Whale Detection** | Large trade tracking with trader context |

---

## Quick Start

### Stream Data (No Database)

```bash
# Install CLI
brew install streamingfast/tap/substreams

# Authenticate
substreams auth

# Stream order fills
substreams run https://spkg.io/PaulieB14/polymarket-pnl-v1.2.0.spkg \
  map_order_fills \
  -e polygon.substreams.pinax.network:443 \
  -s 65000000 -t +1000

# Stream user P&L
substreams run https://spkg.io/PaulieB14/polymarket-pnl-v1.2.0.spkg \
  map_user_pnl \
  -e polygon.substreams.pinax.network:443 \
  -s 65000000 -t +1000
```

### Sink to PostgreSQL (Required for P&L)

> **Important:** P&L requires accumulated state. Use the SQL sink for accurate calculations.

```bash
# Install sink
brew install streamingfast/tap/substreams-sink-sql

# Create database
createdb polymarket_pnl

# Setup schema
substreams-sink-sql setup \
  "psql://localhost:5432/polymarket_pnl?sslmode=disable" \
  https://spkg.io/PaulieB14/polymarket-pnl-v1.2.0.spkg

# Run sink (start from beginning for full history)
substreams-sink-sql run \
  "psql://localhost:5432/polymarket_pnl?sslmode=disable" \
  https://spkg.io/PaulieB14/polymarket-pnl-v1.2.0.spkg \
  -e polygon.substreams.pinax.network:443
```

### Query Your Data

```sql
-- Top traders by P&L
SELECT * FROM leaderboard_pnl LIMIT 20;

-- Whale trades with trader stats
SELECT * FROM whale_trades;

-- User positions
SELECT * FROM user_positions
WHERE user_address = '0x...' AND quantity > 0;

-- Daily stats
SELECT date, total_volume, total_trades
FROM daily_stats ORDER BY date DESC;
```

---

## Architecture

```
                    Polygon Blockchain
                           │
                           ▼
              ┌─────────────────────────┐
              │     Firehose Blocks     │
              └─────────────────────────┘
                           │
         ┌─────────────────┼─────────────────┐
         ▼                 ▼                 ▼
┌─────────────────┐ ┌─────────────────┐ ┌─────────────────┐
│ map_order_fills │ │map_token_transf │ │map_usdc_transf  │
│  (CTF + NegRisk)│ │    (ERC1155)    │ │    (USDC)       │
└─────────────────┘ └─────────────────┘ └─────────────────┘
         │                 │                   │
         └─────────────────┼───────────────────┘
                           ▼
              ┌─────────────────────────┐
              │    STORES (State)       │
              │  • user_positions       │
              │  • user_cost_basis      │
              │  • user_realized_pnl    │
              │  • latest_prices        │
              └─────────────────────────┘
                           │
                           ▼
              ┌─────────────────────────┐
              │      map_user_pnl       │
              │   (Computed Analytics)  │
              └─────────────────────────┘
                           │
                           ▼
              ┌─────────────────────────┐
              │         db_out          │
              │      (SQL Sink)         │
              └─────────────────────────┘
                           │
                           ▼
              ┌─────────────────────────┐
              │       PostgreSQL        │
              └─────────────────────────┘
```

---

## Modules

### Layer 1: Event Extraction

| Module | Description |
|--------|-------------|
| `map_order_fills` | OrderFilled events from CTF & NegRisk exchanges |
| `map_token_transfers` | ERC1155 TransferSingle events |
| `map_usdc_transfers` | USDC transfer events |

### Layer 2: State Stores

| Store | Key | Description |
|-------|-----|-------------|
| `store_user_positions` | `{user}:{token}` | Position quantities |
| `store_user_cost_basis` | `{user}:{token}` | Total cost basis |
| `store_user_realized_pnl` | `{user}` | Realized P&L |
| `store_user_volume` | `{user}` | Trading volume |
| `store_user_trade_count` | `{user}` | Trade count |
| `store_market_volume` | `{token}` | Market volume |
| `store_latest_prices` | `{token}` | Latest prices |

### Layer 3: Analytics

| Module | Description |
|--------|-------------|
| `map_user_pnl` | Real-time P&L calculations |
| `map_market_stats` | Market-level statistics |

### Layer 4: Sink

| Module | Description |
|--------|-------------|
| `db_out` | Database changes for SQL sink |

---

## Database Schema

### Tables

| Table | Description |
|-------|-------------|
| `trades` | All order fills with price, amount, side |
| `user_pnl` | Aggregated P&L per user |
| `user_positions` | Current positions with cost basis |
| `markets` | Market statistics |
| `daily_stats` | Daily aggregates |

### Views

| View | Description |
|------|-------------|
| `leaderboard_pnl` | Top 1000 by P&L |
| `leaderboard_volume` | Top 1000 by volume |
| `whale_trades` | Trades >$10K |

---

## Contract Addresses

### CLOB v1 (legacy — historical fills only after the 2026-04-28 cutover)

| Contract | Address | Start Block |
|----------|---------|-------------|
| CTF Exchange v1 | `0x4bfb41d5b3570defd03c39a9a4d8de6bd8b8982e` | 33,605,403 |
| NegRisk Exchange v1 | `0xC5d563A36AE78145C45a50134d48A1215220f80a` | 50,505,492 |

### CLOB v2 (deployed 2026-03-31 by Polymarket Deployer 1, cutover 2026-04-28)

| Contract | Address |
|----------|---------|
| CTF Exchange V2 | `0xE111180000d2663C0091e4f400237545B87B996B` |
| NegRisk CTF Exchange V2 | `0xe2222d279d744050d28e00520010520000310F59` |

### Other (unchanged)

| Contract | Address | Start Block |
|----------|---------|-------------|
| Conditional Tokens | `0x4D97DCd97eC945f40cF65F87097ACe5EA0476045` | 4,023,686 |
| USDC | `0x2791Bca1f2de4661ED88A30C99A7a9449Aa84174` | 4,023,686 |

---

## Build from Source

```bash
# Clone
git clone https://github.com/PaulieB14/Polymarket-P-L-Substreams
cd Polymarket-P-L-Substreams

# Build
substreams build

# Test
substreams run substreams.yaml map_order_fills \
  -e polygon.substreams.pinax.network:443 \
  -s 65000000 -t +100

# Package & publish
substreams pack substreams.yaml -o polymarket-pnl-v1.2.0.spkg
substreams publish polymarket-pnl-v1.2.0.spkg
```

---

## Why SQL Sink?

P&L calculation requires **state accumulation** over time:

| Without Sink | With SQL Sink |
|--------------|---------------|
| No history | Full history persisted |
| P&L = $0 | Accurate P&L |
| Stateless | Tracks cost basis |
| Demo only | Production ready |

---

## Related

- [polymarket-orderbook-substreams](https://substreams.dev/packages/polymarket-orderbook-substreams/v0.2.0) - Order flow & trader leaderboards

---

## License

MIT
