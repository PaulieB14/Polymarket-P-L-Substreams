# Polymarket P&L Substreams

<p align="center">
  <img src="./polymarket-pnl-icon.png" alt="Polymarket P&L" width="120"/>
</p>

<p align="center">
  <strong>Real-time Profit & Loss tracking for Polymarket prediction markets on Polygon</strong>
</p>

<p align="center">
  <a href="https://substreams.dev/packages/polymarket-pnl/v2.0.1">
    <img src="https://img.shields.io/badge/substreams.dev-v2.0.1-blue" alt="Substreams"/>
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

Per-wallet positions, average-cost basis, realized and unrealized P&L, volume, fees, win rate, resolutions, redemptions and Protocol V2 migrations for Polymarket on Polygon. It covers CLOB v1, CLOB v2 and Protocol V2 (ExchangeV3 + PositionManager, pUSD collateral).

## What's new in v2.0.1 (breaking)

| Area | v1.x | v2.0.1 |
|---|---|---|
| Fills | `map_order_fills` produced **no fills**: its blockFilter joined terms with `OR` (the index query language uses `\|\|`) and used checksummed addresses (index keys are lowercase) | `map_events` uses a lowercase, `\|\|`-joined filter. All five exchanges, including **ExchangeV3** `0xe3333700…` |
| Positions | TransferSingle from any ERC-1155 contract | TransferSingle **and TransferBatch**, only from Conditional Tokens `0x4D97…` and PositionManager `0x006F54F7…` |
| Collateral | USDC.e | USDC.e **and pUSD** (`map_collateral_transfers`) |
| Cost basis | none (avg price = last fill) | Average buy cost per (wallet, token), including splits. Realized P&L on sells, merges and redemptions |
| Stats | volume / win rate / first trade not tracked | volume, fees, trades, win/loss counts, win_rate, largest win/loss, first/last trade |
| New events | – | FeeCharged, CTF ConditionResolution, V2 ConditionResolved, UMA v3.1/v2 QuestionResolved, CTF PayoutRedemption, V2 PositionRedeemed, AutoRedeemer redemptions, V2 PositionMigrated (carries cost basis to the new position) |
| Ordering | HashSet (non-deterministic) | BTreeSet/BTreeMap (deterministic) |
| Excluded addresses | 2 | All Polymarket protocol contracts (exchanges, ledgers, modules, adapters, router, AutoRedeemer, UMA v3.1) |

### Migrating from v1
- Proto package is now `pnl.v2`. Module names changed: `map_order_fills`, `map_token_transfers`, `map_usdc_transfers` and the seven old stores become `map_events`, `map_collateral_transfers`, `store_position_totals`, `map_migration_carry`, `store_migration_carry`, `map_realized`, `store_pnl_totals`, `store_pnl_max`, `store_pnl_min` and `store_latest_prices`. `map_user_pnl`, `map_market_stats` and `db_out` keep their names, with new outputs.
- The SQL schema is new. Create a fresh database (or drop the old tables) and run `substreams-sink-sql setup` again. Amounts are now **raw 6-decimal base units** (divide by 1e6). `trades.amount` is replaced by `shares` / `collateral` / `fee`.
- New tables: `fees`, `resolutions`, `redemptions`, `migrations`. New views: `v_user_positions_live`, `v_user_pnl_live` (unrealized P&L at the latest traded price).

## Quick start

```bash
substreams run polymarket-pnl@v2.0.1 map_events -s -1000 -t 0 -e polygon.streamingfast.io:443
substreams run polymarket-pnl@v2.0.1 map_user_pnl -s -1000 -t 0 -e polygon.streamingfast.io:443

# Postgres (needed for full-history P&L)
substreams-sink-sql setup "psql://user:pass@localhost:5432/pnl?sslmode=disable" polymarket-pnl@v2.0.1
substreams-sink-sql run   "psql://user:pass@localhost:5432/pnl?sslmode=disable" polymarket-pnl@v2.0.1
```

```sql
-- leaderboard (USD)
SELECT user_address, realized_pnl/1e6 AS realized, unrealized_pnl/1e6 AS unrealized, total_pnl/1e6 AS total,
       total_volume/1e6 AS volume, win_rate
FROM v_user_pnl_live ORDER BY total_pnl DESC LIMIT 50;
```

## Modules

```
sf.ethereum.type.v2.Block
 ├─ map_events ─────────────┬─ store_position_totals ─ map_migration_carry ─ store_migration_carry
 │                          ├─ map_realized ─┬─ store_pnl_totals / store_pnl_max / store_pnl_min
 │                          ├─ store_latest_prices
 │                          ├─ map_user_pnl ─┐
 │                          ├─ map_market_stats ─┤
 │                          └────────────────────┴─ db_out
 └─ map_collateral_transfers
```

All modules start at block 33,605,403. The block filters use `ethcommon:index_events`.

## P&L methodology

- **Fill attribution.** Each `OrderFilled` belongs to its `maker` (the order owner). The taker order's own fill has the exchange as `taker`, so every participant is counted exactly once. From Polymarket `ctf-exchange` / `ctf-exchange-v2` `Trading.sol`:
  - v1: the owner receives `taking − fee` (shares on a BUY, collateral on a SELL).
  - v2 and V3: the fee is collateral. A BUY pays `makerAmountFilled + fee`; a SELL receives `takerAmountFilled − fee`.
- **Average cost** per (wallet, token) = cumulative buy collateral ÷ cumulative bought shares. Buys include binary splits made by the wallet, priced at collateral ÷ minted shares. This is the average over *all* buys, not reduced by sells, and it equals standard average cost when buys precede sells.
- **Realized P&L** = proceeds − shares × average cost, on every sell, merge and redemption. A realizing event with P&L > 0 is a win, < 0 is a loss. `win_rate` = wins ÷ (wins + losses).
- **Unrealized P&L** = held quantity × (last fill price − average cost). The views recompute it at each token's latest price.
- **Volume** per wallet = collateral of its own fills. **Market volume** counts each match once.

### Known limitations
- Stores within a block are read after the whole block is applied: a sell in the same block as a later buy uses the end-of-block average.
- CTF `PayoutRedemption` reports one payout per condition. It is split across the burned tokens pro rata to amount. V2 `PositionRedeemed` is exact per position.
- When a contract pulls tokens and redeems them in one tx, the redemption is attributed to the sender of those tokens. Contracts that hold tokens across transactions (for example the undocumented `0xf3cfb6a6…`) appear as their own wallet.
- Splits and merges are counted only for the part the stakeholder keeps (or brought in) within the tx, so routers and adapters net to zero. Adapter-level conversions (NegRiskAdapter convert, collateral adapters) are not priced.
- Combinatorial-market lifecycle events and NegRiskAdapter-specific redemptions are not decoded. UMA `QuestionResolved` is only read from the documented v3.1 / v2 adapters; CTF `ConditionResolution` is the canonical resolution.
- Balances start at block 33,605,403. Positions opened earlier have quantity but no cost basis.

## Contract addresses

From https://docs.polymarket.com/resources/contracts. The v1 exchanges come from github.com/Polymarket/ctf-exchange. All of them are in `src/addresses.rs`.

| Contract | Address |
|---|---|
| CTF Exchange v1 / Neg Risk Exchange v1 | `0x4bfb41d5b3570defd03c39a9a4d8de6bd8b8982e` / `0xC5d563A36AE78145C45a50134d48A1215220f80a` |
| CTF Exchange v2 / Neg Risk Exchange v2 | `0xE111180000d2663C0091e4f400237545B87B996B` / `0xe2222d279d744050d28e00520010520000310F59` |
| ExchangeV3 (Protocol V2) | `0xe3333700cA9d93003F00f0F71f8515005F6c00Aa` |
| Conditional Tokens / PositionManager | `0x4D97DCd97eC945f40cF65F87097ACe5EA0476045` / `0x006F54F7F9A22e0000CC2Ab60031000000aE9FEF` |
| pUSD / USDC.e | `0xC011a7E12a19f7B1f670d46F03B03f3342E82DFB` / `0x2791Bca1f2de4661ED88A30C99A7a9449Aa84174` |
| BinaryModule / NegRiskModule / CombinatorialModule | `0x1000008dD9001B968442c1000017eaE6E0dA00Ba` / `0x200000900045e3B6259600682756002200028933` / `0x30000034706C7d8e12009DAB006Be20000c031A8` |
| AutoRedeemer / Router | `0xa1200000d0002264C9a1698e001292D00E1b00af` / `0x12121212006e4CD160D18e3f00711DA5c3372600` |
| UmaCtfAdapter v3.1 / v2 | `0x157Ce2d672854c848c9b79C49a8Cc6cc89176a49` / `0x6A9D222616C90FcA5754cd1333cFD9b7fb6a4F74` |

## Build

```bash
substreams build          # CLI >= 1.25; src/pb is generated (buffa)
cargo test --lib          # unit tests: fill/fee semantics, TransferBatch, cost basis, formatting
```

## License

MIT
