-- Polymarket P&L Substreams v2.0.0 - PostgreSQL schema
-- Amounts (shares, collateral, fee, pnl, volume) are raw base units (6 decimals): divide by 1e6.
-- Prices are decimals 0..1.
-- Setup: substreams-sink-sql setup "postgres://..." polymarket-pnl-v2.0.0.spkg

CREATE TABLE IF NOT EXISTS trades (
    id               VARCHAR(80) PRIMARY KEY,   -- tx_hash-log_index
    block_number     BIGINT NOT NULL,
    block_timestamp  TIMESTAMP NOT NULL,
    tx_hash          VARCHAR(66) NOT NULL,
    log_index        INTEGER NOT NULL,
    exchange         VARCHAR(16) NOT NULL,      -- ctf_v1 | neg_risk_v1 | ctf_v2 | neg_risk_v2 | exchange_v3
    position_system  VARCHAR(8) NOT NULL,       -- ctf | pm_v2
    order_hash       VARCHAR(66),
    maker            VARCHAR(42) NOT NULL,      -- order owner this fill belongs to
    taker            VARCHAR(42) NOT NULL,
    side             VARCHAR(4) NOT NULL,       -- buy | sell (maker's side)
    token_id         VARCHAR(78) NOT NULL,
    shares           NUMERIC(78, 0) NOT NULL,
    collateral       NUMERIC(78, 0) NOT NULL,   -- paid incl. fee (buy) / received net of fee (sell)
    fee              NUMERIC(78, 0) NOT NULL,
    price            NUMERIC(20, 6) NOT NULL,
    builder          VARCHAR(66),
    metadata         VARCHAR(66)
);
CREATE INDEX IF NOT EXISTS idx_trades_block ON trades(block_number);
CREATE INDEX IF NOT EXISTS idx_trades_maker ON trades(maker);
CREATE INDEX IF NOT EXISTS idx_trades_token ON trades(token_id);
CREATE INDEX IF NOT EXISTS idx_trades_ts ON trades(block_timestamp);

CREATE TABLE IF NOT EXISTS fees (
    id VARCHAR(80) PRIMARY KEY, block_number BIGINT NOT NULL, block_timestamp TIMESTAMP NOT NULL,
    tx_hash VARCHAR(66) NOT NULL, exchange VARCHAR(16) NOT NULL, receiver VARCHAR(42) NOT NULL,
    amount NUMERIC(78, 0) NOT NULL
);
CREATE INDEX IF NOT EXISTS idx_fees_receiver ON fees(receiver);

CREATE TABLE IF NOT EXISTS resolutions (
    id VARCHAR(80) PRIMARY KEY, block_number BIGINT NOT NULL, block_timestamp TIMESTAMP NOT NULL,
    tx_hash VARCHAR(66) NOT NULL,
    source VARCHAR(24) NOT NULL,          -- ctf | binary_module | neg_risk_module | combinatorial_module | uma_v3_1 | uma_v2
    condition_id VARCHAR(66) NOT NULL,    -- UMA rows: questionID
    oracle VARCHAR(42), settled_price VARCHAR(78),
    payouts TEXT                           -- comma-separated payout numerators
);
CREATE INDEX IF NOT EXISTS idx_resolutions_condition ON resolutions(condition_id);

CREATE TABLE IF NOT EXISTS redemptions (
    id VARCHAR(80) PRIMARY KEY, block_number BIGINT NOT NULL, block_timestamp TIMESTAMP NOT NULL,
    tx_hash VARCHAR(66) NOT NULL, source VARCHAR(16) NOT NULL, -- ctf | pm_v2 | auto_redeemer
    holder VARCHAR(42), condition_id VARCHAR(66), position_id VARCHAR(78),
    amount NUMERIC(78, 0), payout NUMERIC(78, 0)
);
CREATE INDEX IF NOT EXISTS idx_redemptions_holder ON redemptions(holder);

CREATE TABLE IF NOT EXISTS migrations (
    id VARCHAR(80) PRIMARY KEY, block_number BIGINT NOT NULL, block_timestamp TIMESTAMP NOT NULL,
    tx_hash VARCHAR(66) NOT NULL, holder VARCHAR(42) NOT NULL, condition_id VARCHAR(66),
    position_id VARCHAR(78) NOT NULL, outcome_index VARCHAR(8), amount NUMERIC(78, 0) NOT NULL,
    legacy_token_id VARCHAR(78)
);

CREATE TABLE IF NOT EXISTS user_pnl (
    user_address    VARCHAR(42) PRIMARY KEY,
    realized_pnl    NUMERIC(78, 0) NOT NULL DEFAULT 0,
    total_volume    NUMERIC(78, 0) NOT NULL DEFAULT 0,
    total_trades    BIGINT NOT NULL DEFAULT 0,
    total_fees_paid NUMERIC(78, 0) NOT NULL DEFAULT 0,
    win_count       BIGINT NOT NULL DEFAULT 0,   -- realizing events (sell/merge/redeem) with pnl > 0
    loss_count      BIGINT NOT NULL DEFAULT 0,
    win_rate        NUMERIC(6, 4) NOT NULL DEFAULT 0,
    largest_win     NUMERIC(78, 0) NOT NULL DEFAULT 0,
    largest_loss    NUMERIC(78, 0) NOT NULL DEFAULT 0,
    first_trade_at  TIMESTAMP,
    last_trade_at   TIMESTAMP,
    last_block      BIGINT NOT NULL
);
CREATE INDEX IF NOT EXISTS idx_user_pnl_realized ON user_pnl(realized_pnl DESC);
CREATE INDEX IF NOT EXISTS idx_user_pnl_volume ON user_pnl(total_volume DESC);

CREATE TABLE IF NOT EXISTS user_positions (
    user_address    VARCHAR(42) NOT NULL,
    token_id        VARCHAR(78) NOT NULL,
    quantity        NUMERIC(78, 0) NOT NULL DEFAULT 0,
    bought_shares   NUMERIC(78, 0) NOT NULL DEFAULT 0,
    bought_cost     NUMERIC(78, 0) NOT NULL DEFAULT 0,
    avg_entry_price NUMERIC(20, 6) NOT NULL DEFAULT 0,
    cost_basis      NUMERIC(78, 0) NOT NULL DEFAULT 0,  -- quantity x avg_entry_price
    realized_pnl    NUMERIC(78, 0) NOT NULL DEFAULT 0,
    last_price      NUMERIC(20, 6) NOT NULL DEFAULT 0,  -- token price when the row was last written
    unrealized_pnl  NUMERIC(78, 0) NOT NULL DEFAULT 0,  -- as of last_block; see v_user_positions_live
    last_block      BIGINT NOT NULL,
    PRIMARY KEY (user_address, token_id)
);
CREATE INDEX IF NOT EXISTS idx_positions_token ON user_positions(token_id);
CREATE INDEX IF NOT EXISTS idx_positions_open ON user_positions(user_address) WHERE quantity > 0;

CREATE TABLE IF NOT EXISTS markets (
    token_id   VARCHAR(78) PRIMARY KEY,
    exchange   VARCHAR(16) NOT NULL,
    volume     NUMERIC(78, 0) NOT NULL DEFAULT 0,   -- counted once per match
    trades     BIGINT NOT NULL DEFAULT 0,
    last_price NUMERIC(20, 6) NOT NULL DEFAULT 0,
    last_block BIGINT NOT NULL
);

-- Unrealized P&L at each token's latest traded price (positions are only rewritten when they change).
CREATE OR REPLACE VIEW v_user_positions_live AS
SELECT p.user_address, p.token_id, p.quantity, p.avg_entry_price, p.cost_basis, p.realized_pnl,
       m.last_price,
       FLOOR(p.quantity * m.last_price) - p.cost_basis AS unrealized_pnl
FROM user_positions p
LEFT JOIN markets m ON m.token_id = p.token_id;

CREATE OR REPLACE VIEW v_user_pnl_live AS
SELECT u.*, COALESCE(x.unrealized, 0) AS unrealized_pnl, u.realized_pnl + COALESCE(x.unrealized, 0) AS total_pnl
FROM user_pnl u
LEFT JOIN (SELECT user_address, SUM(unrealized_pnl) AS unrealized FROM v_user_positions_live
           WHERE quantity > 0 GROUP BY user_address) x ON x.user_address = u.user_address;
