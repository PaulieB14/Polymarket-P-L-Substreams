//! polymarket-pnl v2.0.0
//!
//! One pass over each block (`map_events`) extracts fills from all five Polymarket exchanges,
//! ERC-1155 balance changes from both position ledgers (Conditional Tokens and
//! PositionManager), fees, resolutions, redemptions and migrations, and derives a per-user
//! cost-basis ledger. Stores then aggregate the ledger into positions and P&L.
//!
//! Cost-basis model (documented in README "P&L methodology"):
//! - average cost = cumulative buy collateral / cumulative bought shares, per (user, token);
//!   buys include binary splits priced at collateral / minted shares;
//! - realized P&L on a sell, merge or redemption = proceeds - shares x average cost;
//! - unrealized P&L per position = held shares x (last trade price - average cost).
//! All amounts are in collateral base units (6 decimals) unless stated otherwise.

mod abi;
mod addresses;
mod pb;
#[cfg(test)]
mod tests;

use std::collections::{BTreeMap, BTreeSet};
use std::str::FromStr;

use addresses as A;
use buffa_types::google::protobuf::Timestamp;
use pb::pnl::v2 as pnl;
use substreams::prelude::*;
use substreams::scalar::BigInt;
use substreams::store::{
    StoreAdd, StoreAddBigInt, StoreGet, StoreGetBigInt, StoreGetString, StoreMax, StoreMaxBigInt,
    StoreMin, StoreMinBigInt, StoreNew, StoreSet, StoreSetString,
};
use substreams::Hex;
use substreams_database_change::pb::sf::substreams::sink::database::v1::DatabaseChanges;
use substreams_database_change::tables::Tables;
use substreams_ethereum::pb::eth::v2 as eth;

substreams_ethereum::init!();

pub const PRICE_SCALE: u64 = 1_000_000;

// ---------------------------------------------------------------------------
// helpers
// ---------------------------------------------------------------------------

pub fn bi(s: &str) -> BigInt {
    BigInt::from_str(s).unwrap_or_default()
}

/// price in micro-units (1.0 == 1_000_000), rounded down.
pub fn price_e6(collateral: &BigInt, shares: &BigInt) -> BigInt {
    if shares.is_zero() {
        return BigInt::zero();
    }
    collateral.clone() * BigInt::from(PRICE_SCALE) / shares.clone()
}

pub fn fmt_price(p_e6: &BigInt) -> String {
    let neg = p_e6 < &BigInt::zero();
    let a = if neg { BigInt::zero() - p_e6.clone() } else { p_e6.clone() };
    let s = BigInt::from(PRICE_SCALE);
    let int = a.clone() / s.clone();
    let frac = a - int.clone() * s;
    format!("{}{}.{:06}", if neg { "-" } else { "" }, int, frac.to_u64())
}

/// cost of `shares` at the average cost bought_cost / bought_shares (0 if nothing bought).
pub fn avg_cost(shares: &BigInt, bought_shares: &BigInt, bought_cost: &BigInt) -> BigInt {
    if bought_shares.is_zero() {
        return BigInt::zero();
    }
    shares.clone() * bought_cost.clone() / bought_shares.clone()
}

pub fn win_rate(wins: u64, losses: u64) -> String {
    if wins + losses == 0 {
        return "0.0000".to_string();
    }
    let bp = wins * 10_000 / (wins + losses);
    format!("{}.{:04}", bp / 10_000, bp % 10_000)
}

pub fn unix_to_timestamp(secs: i64) -> String {
    let days_since_epoch = secs.div_euclid(86400);
    let tod = secs.rem_euclid(86400);
    // civil-from-days (Howard Hinnant)
    let z = days_since_epoch + 719_468;
    let era = z.div_euclid(146_097);
    let doe = z.rem_euclid(146_097);
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146_096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = if m <= 2 { y + 1 } else { y };
    format!("{:04}-{:02}-{:02} {:02}:{:02}:{:02}", y, m, d, tod / 3600, (tod % 3600) / 60, tod % 60)
}

fn addr(bytes: &[u8]) -> String {
    format!("0x{}", Hex(bytes))
}

fn pos_key(user: &str, token: &str, field: &str) -> String {
    format!("p:{}:{}:{}", user, token, field)
}

fn user_key(user: &str, field: &str) -> String {
    format!("u:{}:{}", user, field)
}

// ---------------------------------------------------------------------------
// Layer 1: map_events
// ---------------------------------------------------------------------------

struct Xfer {
    log_index: u32,
    batch_index: u32,
    ledger: &'static str,
    operator: String,
    from: String,
    to: String,
    id: BigInt,
    value: BigInt,
}

#[substreams::handlers::map]
fn map_events(blk: eth::Block) -> Result<pnl::Events, substreams::errors::Error> {
    let ts = blk.timestamp_seconds() as i64;
    let mut out = pnl::Events {
        block_number: blk.number,
        block_timestamp: Timestamp { seconds: ts, nanos: 0, ..Default::default() }.into(),
        ..Default::default()
    };

    for trx in blk.transactions() {
        let Some(receipt) = trx.receipt.as_option() else { continue };
        let tx_hash = addr(&trx.hash);
        let mut xfers: Vec<Xfer> = vec![];
        // tx-local context for ledger derivation
        let mut splits: BTreeMap<String, BigInt> = BTreeMap::new(); // stakeholder -> collateral
        let mut merges: BTreeMap<String, BigInt> = BTreeMap::new();
        let mut ctf_redeems: Vec<(String, BigInt, String, u32)> = vec![]; // (redeemer, payout, condition, log)
        let mut pm_redeems: Vec<(String, String, BigInt, BigInt, String, u32)> = vec![]; // (initiator, recipient, positionId, amount, payout) + log
        let mut ar_holders: Vec<(String, String)> = vec![]; // (holder, condition/position id) from AutoRedeemer
        let mut migrations: Vec<(String, String, BigInt, String, BigInt, u32)> = vec![];

        for log in receipt.logs.iter() {
            let a = addr(&log.address);
            let li = log.index;
            let id = format!("{}-{}", tx_hash, li);

            if let Some((exchange, system)) = A::exchange_of(&a) {
                if let Some(f) = abi::decode_fill(log) {
                    let (shares, collateral) = f.shares_and_collateral();
                    let fill = pnl::OrderFill {
                        id: id.clone(),
                        tx_hash: tx_hash.clone(),
                        log_index: li,
                        block_number: blk.number,
                        timestamp: Timestamp { seconds: ts, nanos: 0, ..Default::default() }.into(),
                        exchange: exchange.to_string(),
                        position_system: system.to_string(),
                        order_hash: f.order_hash.clone(),
                        maker: f.maker.clone(),
                        taker: f.taker.clone(),
                        side: if f.buy { "buy" } else { "sell" }.to_string(),
                        token_id: f.token_id.to_string(),
                        shares: shares.to_string(),
                        collateral: collateral.to_string(),
                        fee: f.fee.to_string(),
                        price: fmt_price(&price_e6(&collateral, &shares)),
                        maker_amount_filled: f.maker_amount.to_string(),
                        taker_amount_filled: f.taker_amount.to_string(),
                        builder: f.builder.clone(),
                        metadata: f.metadata.clone(),
                        ..Default::default()
                    };
                    if !A::is_excluded(&f.maker) && !shares.is_zero() {
                        out.ledger.push(pnl::LedgerEntry {
                            id: id.clone(),
                            user: f.maker.clone(),
                            token_id: fill.token_id.clone(),
                            kind: fill.side.clone(),
                            shares: fill.shares.clone(),
                            collateral: fill.collateral.clone(),
                            fee: fill.fee.clone(),
                            ..Default::default()
                        });
                    }
                    out.fills.push(fill);
                } else if abi::is(log, &abi::FEE_CHARGED) {
                    if let (Some(rcv), Some(amt)) = (abi::topic_addr(log, 1), abi::uint(&log.data, 0)) {
                        out.fees.push(pnl::Fee {
                            id: id.clone(), tx_hash: tx_hash.clone(), log_index: li,
                            exchange: exchange.to_string(), receiver: rcv, amount: amt.to_string(),
                            ..Default::default()
                        });
                    }
                }
                continue;
            }

            match a.as_str() {
                A::CONDITIONAL_TOKENS | A::POSITION_MANAGER => {
                    let ledger = if a == A::CONDITIONAL_TOKENS { "ctf" } else { "pm_v2" };
                    if let Some((op, from, to, items)) = abi::decode_erc1155(log) {
                        for (k, (tid, val)) in items.into_iter().enumerate() {
                            xfers.push(Xfer {
                                log_index: li, batch_index: k as u32, ledger,
                                operator: op.clone(), from: from.clone(), to: to.clone(), id: tid, value: val,
                            });
                        }
                    } else if a == A::CONDITIONAL_TOKENS {
                        if abi::is(log, &abi::CONDITION_RESOLUTION) {
                            out.resolutions.push(pnl::Resolution {
                                id: id.clone(), tx_hash: tx_hash.clone(), log_index: li,
                                source: "ctf".into(),
                                condition_id: abi::topic_hex(log, 1).unwrap_or_default(),
                                oracle: abi::topic_addr(log, 2).unwrap_or_default(),
                                payouts: abi::uint_array(&log.data, 1).unwrap_or_default().iter().map(|p| p.to_string()).collect(),
                                ..Default::default()
                            });
                        } else if abi::is(log, &abi::PAYOUT_REDEMPTION) {
                            if let (Some(r), Some(cond), Some(payout)) = (abi::topic_addr(log, 1), abi::word(&log.data, 0), abi::uint(&log.data, 2)) {
                                ctf_redeems.push((r, payout, addr(cond), li));
                            }
                        } else if abi::is(log, &abi::CTF_POSITION_SPLIT) || abi::is(log, &abi::CTF_POSITIONS_MERGE) {
                            if let (Some(s), Some(amount)) = (abi::topic_addr(log, 1), abi::uint(&log.data, 2)) {
                                let m = if abi::is(log, &abi::CTF_POSITION_SPLIT) { &mut splits } else { &mut merges };
                                let e = m.entry(s).or_insert_with(BigInt::zero);
                                *e = e.clone() + amount;
                            }
                        }
                    }
                }
                A::PUSD | A::USDC_E => {}
                A::BINARY_MODULE | A::NEG_RISK_MODULE | A::COMBINATORIAL_MODULE => {
                    let src = match a.as_str() { A::BINARY_MODULE => "binary_module", A::NEG_RISK_MODULE => "neg_risk_module", _ => "combinatorial_module" };
                    if abi::is(log, &abi::CONDITION_RESOLVED) {
                        out.resolutions.push(pnl::Resolution {
                            id: id.clone(), tx_hash: tx_hash.clone(), log_index: li, source: src.into(),
                            condition_id: abi::topic_hex(log, 1).unwrap_or_default(),
                            payouts: abi::uint_array(&log.data, 0).unwrap_or_default().iter().map(|p| p.to_string()).collect(),
                            ..Default::default()
                        });
                    } else if abi::is(log, &abi::POSITION_REDEEMED) {
                        if let (Some(ini), Some(pid), Some(rcp), Some(amt), Some(pay)) = (
                            abi::topic_addr(log, 1), abi::topic_uint(log, 2), abi::topic_addr(log, 3),
                            abi::uint(&log.data, 0), abi::uint(&log.data, 1),
                        ) {
                            pm_redeems.push((ini, rcp, pid, amt, pay.to_string(), li));
                        }
                    } else if abi::is(log, &abi::V2_POSITIONS_SPLIT) || abi::is(log, &abi::V2_POSITIONS_MERGED) {
                        // PositionsSplit(initiator, conditionId, recipient0, recipient1, amount)
                        // PositionsMerged(initiator, conditionId, recipient, amount)
                        let split = abi::is(log, &abi::V2_POSITIONS_SPLIT);
                        let holder = if split { abi::topic_addr(log, 3) } else { abi::topic_addr(log, 1) };
                        let amount = if split { abi::uint(&log.data, 1) } else { abi::uint(&log.data, 0) };
                        if let (Some(h), Some(amt)) = (holder, amount) {
                            let m = if split { &mut splits } else { &mut merges };
                            let e = m.entry(h).or_insert_with(BigInt::zero);
                            *e = e.clone() + amt;
                        }
                    } else if abi::is(log, &abi::POSITION_MIGRATED) {
                        if let (Some(from), Some(cond), Some(pid), Some(oi), Some(amt)) = (
                            abi::topic_addr(log, 1), abi::topic_hex(log, 2), abi::topic_uint(log, 3),
                            abi::uint(&log.data, 0), abi::uint(&log.data, 1),
                        ) {
                            migrations.push((from, cond, pid, oi.to_string(), amt, li));
                        }
                    }
                }
                A::AUTO_REDEEMER => {
                    if abi::is(log, &abi::AR_REDEMPTION) || abi::is(log, &abi::AR_BINARY_REDEMPTION) || abi::is(log, &abi::AR_NEG_RISK_REDEMPTION) {
                        if let (Some(from), Some(key), Some(pay)) = (abi::topic_addr(log, 1), abi::topic_hex(log, 2), abi::uint(&log.data, 0)) {
                            ar_holders.push((from.clone(), key.clone()));
                            let pm = abi::is(log, &abi::AR_REDEMPTION);
                            out.redemptions.push(pnl::Redemption {
                                id: id.clone(), tx_hash: tx_hash.clone(), log_index: li, source: "auto_redeemer".into(),
                                holder: from,
                                condition_id: if pm { String::new() } else { key.clone() },
                                position_id: if pm { abi::topic_uint(log, 2).map(|v| v.to_string()).unwrap_or_default() } else { String::new() },
                                payout: pay.to_string(),
                                ..Default::default()
                            });
                        }
                    }
                }
                A::UMA_ADAPTER_V3_1 | A::UMA_ADAPTER_V2 => {
                    if abi::is(log, &abi::QUESTION_RESOLVED) {
                        out.resolutions.push(pnl::Resolution {
                            id: id.clone(), tx_hash: tx_hash.clone(), log_index: li,
                            source: if a == A::UMA_ADAPTER_V3_1 { "uma_v3_1" } else { "uma_v2" }.into(),
                            condition_id: abi::topic_hex(log, 1).unwrap_or_default(),
                            settled_price: abi::topic_uint(log, 2).map(|v| v.to_string()).unwrap_or_default(),
                            payouts: abi::uint_array(&log.data, 0).unwrap_or_default().iter().map(|p| p.to_string()).collect(),
                            ..Default::default()
                        });
                    }
                }
                _ => {}
            }
        }

        // ---- ERC-1155 transfers ----
        for x in &xfers {
            out.transfers.push(pnl::PositionTransfer {
                tx_hash: tx_hash.clone(), log_index: x.log_index, batch_index: x.batch_index,
                ledger: x.ledger.into(), operator: x.operator.clone(), from_address: x.from.clone(),
                to_address: x.to.clone(), token_id: x.id.to_string(), amount: x.value.to_string(),
                ..Default::default()
            });
        }

        // ---- splits / merges ----
        // Only the part the stakeholder keeps (split) or brought into the tx (merge) is a cost-basis
        // event. Routers and adapters that split and forward tokens in the same tx net to zero.
        for (holder, collateral) in &splits {
            if A::is_excluded(holder) { continue; }
            let minted: Vec<&Xfer> = xfers.iter().filter(|x| x.from == A::ZERO && &x.to == holder).collect();
            let total: BigInt = minted.iter().fold(BigInt::zero(), |s, x| s + x.value.clone());
            for x in minted {
                let out_same: BigInt = xfers.iter().filter(|y| &y.from == holder && y.id == x.id && y.to != A::ZERO)
                    .fold(BigInt::zero(), |s, y| s + y.value.clone());
                let kept = abi::sat_sub(&x.value, &out_same);
                if kept.is_zero() { continue; }
                out.ledger.push(pnl::LedgerEntry {
                    id: format!("{}-{}-{}-split", tx_hash, x.log_index, x.batch_index),
                    user: holder.clone(), token_id: x.id.to_string(), kind: "split".into(),
                    shares: kept.to_string(),
                    collateral: share_of(collateral, &kept, &total).to_string(),
                    fee: "0".into(), ..Default::default()
                });
            }
        }
        for (holder, collateral) in &merges {
            if A::is_excluded(holder) { continue; }
            let burned: Vec<&Xfer> = xfers.iter().filter(|x| x.to == A::ZERO && &x.from == holder).collect();
            let total: BigInt = burned.iter().fold(BigInt::zero(), |s, x| s + x.value.clone());
            for x in burned {
                let in_same: BigInt = xfers.iter().filter(|y| &y.to == holder && y.id == x.id && y.from != A::ZERO)
                    .fold(BigInt::zero(), |s, y| s + y.value.clone());
                let own = abi::sat_sub(&x.value, &in_same);
                if own.is_zero() { continue; }
                out.ledger.push(pnl::LedgerEntry {
                    id: format!("{}-{}-{}-merge", tx_hash, x.log_index, x.batch_index),
                    user: holder.clone(), token_id: x.id.to_string(), kind: "merge".into(),
                    shares: own.to_string(),
                    collateral: share_of(collateral, &own, &total).to_string(),
                    fee: "0".into(), ..Default::default()
                });
            }
        }

        // ---- CTF redemptions: burns from the redeemer; holder is the AutoRedeemer's `from` when it redeemed ----
        let mut by_redeemer: BTreeMap<String, (BigInt, String, u32)> = BTreeMap::new();
        for (r, payout, cond, li) in &ctf_redeems {
            let e = by_redeemer.entry(r.clone()).or_insert((BigInt::zero(), cond.clone(), *li));
            e.0 = e.0.clone() + payout.clone();
        }
        for (redeemer, (payout, cond, li)) in &by_redeemer {
            let ar_holder = if redeemer == A::AUTO_REDEEMER {
                ar_holders.iter().find(|(_, k)| k == cond).map(|(h, _)| h.clone())
            } else {
                None
            };
            let burned: Vec<&Xfer> = xfers.iter().filter(|x| x.ledger == "ctf" && x.to == A::ZERO && &x.from == redeemer).collect();
            let total: BigInt = burned.iter().fold(BigInt::zero(), |s, x| s + x.value.clone());
            // A contract that pulled the tokens in this tx and redeemed them is an intermediary:
            // the holder is whoever sent it those tokens.
            let holder_of = |x: &Xfer| -> String {
                if let Some(h) = &ar_holder { return h.clone(); }
                xfers.iter()
                    .find(|y| &y.to == redeemer && y.id == x.id && y.from != A::ZERO && !A::is_excluded(&y.from))
                    .map(|y| y.from.clone())
                    .unwrap_or_else(|| redeemer.clone())
            };
            let first_holder = burned.first().map(|x| holder_of(x)).or(ar_holder.clone()).unwrap_or_else(|| redeemer.clone());
            out.redemptions.push(pnl::Redemption {
                id: format!("{}-{}", tx_hash, li), tx_hash: tx_hash.clone(), log_index: *li, source: "ctf".into(),
                holder: first_holder, condition_id: cond.clone(), amount: total.to_string(), payout: payout.to_string(),
                ..Default::default()
            });
            for x in burned {
                let holder = holder_of(x);
                if A::is_excluded(&holder) { continue; }
                out.ledger.push(pnl::LedgerEntry {
                    id: format!("{}-{}-{}-redeem", tx_hash, x.log_index, x.batch_index),
                    user: holder, token_id: x.id.to_string(), kind: "redeem".into(),
                    shares: x.value.to_string(), collateral: share_of(payout, &x.value, &total).to_string(),
                    fee: "0".into(), ..Default::default()
                });
            }
        }

        // ---- V2 redemptions: exact per position ----
        for (ini, _rcp, pid, amt, pay, li) in &pm_redeems {
            let pid_s = pid.to_string();
            let holder = ar_holders.iter().find(|(_, k)| bi_hex_eq(k, pid)).map(|(h, _)| h.clone())
                .or_else(|| xfers.iter().find(|x| x.ledger == "pm_v2" && x.to == A::ZERO && x.id == *pid && !A::is_excluded(&x.from)).map(|x| x.from.clone()))
                .unwrap_or_else(|| ini.clone());
            out.redemptions.push(pnl::Redemption {
                id: format!("{}-{}", tx_hash, li), tx_hash: tx_hash.clone(), log_index: *li, source: "pm_v2".into(),
                holder: holder.clone(), position_id: pid_s.clone(), amount: amt.to_string(), payout: pay.clone(),
                ..Default::default()
            });
            if !A::is_excluded(&holder) {
                out.ledger.push(pnl::LedgerEntry {
                    id: format!("{}-{}-redeem", tx_hash, li), user: holder, token_id: pid_s, kind: "redeem".into(),
                    shares: amt.to_string(), collateral: pay.clone(), fee: "0".into(), ..Default::default()
                });
            }
        }

        // ---- migrations: pair with a CTF transfer out of the holder for the same amount ----
        let mut used: BTreeSet<(u32, u32)> = BTreeSet::new();
        for (from, cond, pid, oi, amt, li) in &migrations {
            let legacy = xfers.iter().find(|x| x.ledger == "ctf" && &x.from == from && x.value == *amt && !used.contains(&(x.log_index, x.batch_index)));
            let legacy_id = legacy.map(|x| { used.insert((x.log_index, x.batch_index)); x.id.to_string() }).unwrap_or_default();
            out.migrations.push(pnl::Migration {
                id: format!("{}-{}", tx_hash, li), tx_hash: tx_hash.clone(), log_index: *li, holder: from.clone(),
                condition_id: cond.clone(), position_id: pid.to_string(), outcome_index: oi.clone(),
                amount: amt.to_string(), legacy_token_id: legacy_id.clone(), ..Default::default()
            });
            if legacy_id.is_empty() || A::is_excluded(from) { continue; }
            out.ledger.push(pnl::LedgerEntry {
                id: format!("{}-{}-migrate_out", tx_hash, li), user: from.clone(), token_id: legacy_id.clone(),
                kind: "migrate_out".into(), shares: amt.to_string(), collateral: "0".into(), fee: "0".into(), ..Default::default()
            });
            out.ledger.push(pnl::LedgerEntry {
                id: format!("{}-{}-migrate_in", tx_hash, li), user: from.clone(), token_id: pid.to_string(),
                kind: "migrate_in".into(), shares: amt.to_string(), collateral: "0".into(), fee: "0".into(),
                source_token_id: legacy_id, ..Default::default()
            });
        }
    }
    Ok(out)
}

fn bi_hex_eq(hex_topic: &str, v: &BigInt) -> bool {
    let h = hex_topic.trim_start_matches("0x");
    match substreams::Hex::decode(h) {
        Ok(b) => BigInt::from_unsigned_bytes_be(&b) == *v,
        Err(_) => false,
    }
}

/// part x amount / total, 0 when total is 0.
pub fn share_of(amount: &BigInt, part: &BigInt, total: &BigInt) -> BigInt {
    if total.is_zero() { BigInt::zero() } else { amount.clone() * part.clone() / total.clone() }
}

/// ERC-20 collateral transfers (pUSD and USDC.e). Kept out of map_events because every block has
/// thousands of them.
#[substreams::handlers::map]
fn map_collateral_transfers(blk: eth::Block) -> Result<pnl::Events, substreams::errors::Error> {
    let mut out = pnl::Events { block_number: blk.number, ..Default::default() };
    for log in blk.logs() {
        let a = addr(&log.log.address);
        let token = match a.as_str() { A::PUSD => "pusd", A::USDC_E => "usdc_e", _ => continue };
        if let Some((from, to, amount)) = abi::decode_erc20_transfer(log.log) {
            out.collateral.push(pnl::CollateralTransfer {
                tx_hash: addr(&log.receipt.transaction.hash), log_index: log.log.index, token: token.into(),
                from_address: from, to_address: to, amount: amount.to_string(), ..Default::default()
            });
        }
    }
    Ok(out)
}

// ---------------------------------------------------------------------------
// Layer 2: stores
// ---------------------------------------------------------------------------

/// p:{user}:{token}:{bq|bc|q} — bought shares, bought cost, held quantity.
#[substreams::handlers::store]
fn store_position_totals(ev: pnl::Events, store: StoreAddBigInt) {
    for e in &ev.ledger {
        if e.kind == "buy" || e.kind == "split" {
            store.add(0, pos_key(&e.user, &e.token_id, "bq"), bi(&e.shares));
            store.add(0, pos_key(&e.user, &e.token_id, "bc"), bi(&e.collateral));
        }
    }
    for t in &ev.transfers {
        let v = bi(&t.amount);
        if !A::is_excluded(&t.from_address) {
            store.add(0, pos_key(&t.from_address, &t.token_id, "q"), BigInt::zero() - v.clone());
        }
        if !A::is_excluded(&t.to_address) {
            store.add(0, pos_key(&t.to_address, &t.token_id, "q"), v);
        }
    }
}

/// Moves cost basis from a migrated CTF token to its PositionManager position.
#[substreams::handlers::map]
fn map_migration_carry(ev: pnl::Events, totals: StoreGetBigInt) -> Result<pnl::RealizedEvents, substreams::errors::Error> {
    let mut out = pnl::RealizedEvents { block_number: ev.block_number, ..Default::default() };
    for e in ev.ledger.iter().filter(|e| e.kind == "migrate_in") {
        let bq = totals.get_last(pos_key(&e.user, &e.source_token_id, "bq")).unwrap_or_default();
        let bc = totals.get_last(pos_key(&e.user, &e.source_token_id, "bc")).unwrap_or_default();
        let shares = bi(&e.shares);
        out.events.push(pnl::RealizedEvent {
            entry_id: e.id.clone(), user: e.user.clone(), token_id: e.token_id.clone(), kind: "migrate_in".into(),
            shares: shares.to_string(), cost: avg_cost(&shares, &bq, &bc).to_string(),
            proceeds: e.source_token_id.clone(), realized: "0".into(), ..Default::default()
        });
    }
    Ok(out)
}

/// Same keys as store_position_totals (bq/bc), for cost basis that arrived by migration.
#[substreams::handlers::store]
fn store_migration_carry(carry: pnl::RealizedEvents, store: StoreAddBigInt) {
    for c in &carry.events {
        store.add(0, pos_key(&c.user, &c.token_id, "bq"), bi(&c.shares));
        store.add(0, pos_key(&c.user, &c.token_id, "bc"), bi(&c.cost));
        // the legacy token gives up the same shares and cost
        store.add(0, pos_key(&c.user, &c.proceeds, "bq"), BigInt::zero() - bi(&c.shares));
        store.add(0, pos_key(&c.user, &c.proceeds, "bc"), BigInt::zero() - bi(&c.cost));
    }
}

fn bought(totals: &StoreGetBigInt, carry: &StoreGetBigInt, user: &str, token: &str) -> (BigInt, BigInt) {
    let g = |s: &StoreGetBigInt, f: &str| s.get_last(pos_key(user, token, f)).unwrap_or_default();
    (g(totals, "bq") + g(carry, "bq"), g(totals, "bc") + g(carry, "bc"))
}

/// Realized P&L for every sell, merge and redemption.
#[substreams::handlers::map]
fn map_realized(ev: pnl::Events, totals: StoreGetBigInt, carry: StoreGetBigInt) -> Result<pnl::RealizedEvents, substreams::errors::Error> {
    let mut out = pnl::RealizedEvents { block_number: ev.block_number, ..Default::default() };
    for e in &ev.ledger {
        if !(e.kind == "sell" || e.kind == "merge" || e.kind == "redeem") { continue; }
        let (bq, bc) = bought(&totals, &carry, &e.user, &e.token_id);
        let shares = bi(&e.shares);
        let proceeds = bi(&e.collateral);
        let cost = avg_cost(&shares, &bq, &bc);
        out.events.push(pnl::RealizedEvent {
            entry_id: e.id.clone(), user: e.user.clone(), token_id: e.token_id.clone(), kind: e.kind.clone(),
            shares: shares.to_string(), proceeds: proceeds.to_string(), cost: cost.to_string(),
            realized: (proceeds - cost).to_string(), ..Default::default()
        });
    }
    Ok(out)
}

/// u:{user}:{realized|volume|fees|trades|wins|losses}, p:{user}:{token}:realized,
/// m:{token}:{volume|trades}
#[substreams::handlers::store]
fn store_pnl_totals(ev: pnl::Events, realized: pnl::RealizedEvents, store: StoreAddBigInt) {
    for f in &ev.fills {
        if !A::is_excluded(&f.maker) {
            store.add(0, user_key(&f.maker, "volume"), bi(&f.collateral));
            store.add(0, user_key(&f.maker, "fees"), bi(&f.fee));
            store.add(0, user_key(&f.maker, "trades"), BigInt::one());
        }
        // market volume counts each match once: only the taker order's own fill (taker == exchange)
        if A::exchange_of(&f.taker).is_some() {
            store.add(0, format!("m:{}:volume", f.token_id), bi(&f.collateral));
            store.add(0, format!("m:{}:trades", f.token_id), BigInt::one());
        }
    }
    for r in &realized.events {
        let v = bi(&r.realized);
        store.add(0, user_key(&r.user, "realized"), v.clone());
        store.add(0, pos_key(&r.user, &r.token_id, "realized"), v.clone());
        if v > BigInt::zero() {
            store.add(0, user_key(&r.user, "wins"), BigInt::one());
        } else if v < BigInt::zero() {
            store.add(0, user_key(&r.user, "losses"), BigInt::one());
        }
    }
}

/// u:{user}:largest_win, u:{user}:last_trade
#[substreams::handlers::store]
fn store_pnl_max(ev: pnl::Events, realized: pnl::RealizedEvents, store: StoreMaxBigInt) {
    let ts = ev.block_timestamp.seconds;
    for f in &ev.fills {
        if !A::is_excluded(&f.maker) {
            store.max(0, user_key(&f.maker, "last_trade"), BigInt::from(ts));
        }
    }
    for r in &realized.events {
        store.max(0, user_key(&r.user, "largest_win"), bi(&r.realized));
    }
}

/// u:{user}:largest_loss, u:{user}:first_trade
#[substreams::handlers::store]
fn store_pnl_min(ev: pnl::Events, realized: pnl::RealizedEvents, store: StoreMinBigInt) {
    let ts = ev.block_timestamp.seconds;
    for f in &ev.fills {
        if !A::is_excluded(&f.maker) {
            store.min(0, user_key(&f.maker, "first_trade"), BigInt::from(ts));
        }
    }
    for r in &realized.events {
        store.min(0, user_key(&r.user, "largest_loss"), bi(&r.realized));
    }
}

/// {token} -> last trade price in micro-units.
#[substreams::handlers::store]
fn store_latest_prices(ev: pnl::Events, store: StoreSetString) {
    for f in &ev.fills {
        let p = price_e6(&bi(&f.collateral), &bi(&f.shares));
        store.set(f.log_index as u64, &f.token_id, &p.to_string());
    }
}

// ---------------------------------------------------------------------------
// Layer 3: analytics
// ---------------------------------------------------------------------------

#[substreams::handlers::map]
fn map_user_pnl(
    ev: pnl::Events,
    realized: pnl::RealizedEvents,
    positions: StoreGetBigInt,
    carry: StoreGetBigInt,
    totals: StoreGetBigInt,
    maxes: StoreGetBigInt,
    mins: StoreGetBigInt,
    prices: StoreGetString,
) -> Result<pnl::UserPnLUpdates, substreams::errors::Error> {
    let mut users: BTreeSet<String> = BTreeSet::new();
    let mut pos: BTreeSet<(String, String)> = BTreeSet::new();
    for e in &ev.ledger {
        users.insert(e.user.clone());
        pos.insert((e.user.clone(), e.token_id.clone()));
    }
    for t in &ev.transfers {
        for u in [&t.from_address, &t.to_address] {
            if !A::is_excluded(u) {
                pos.insert((u.clone(), t.token_id.clone()));
            }
        }
    }
    for r in &realized.events {
        users.insert(r.user.clone());
    }

    let mut out = pnl::UserPnLUpdates {
        block_number: ev.block_number,
        block_timestamp: ev.block_timestamp.clone(),
        ..Default::default()
    };
    for u in users {
        let g = |s: &StoreGetBigInt, f: &str| s.get_last(user_key(&u, f));
        let wins = g(&totals, "wins").unwrap_or_default().to_u64();
        let losses = g(&totals, "losses").unwrap_or_default().to_u64();
        out.users.push(pnl::UserPnL {
            user_address: u.clone(),
            realized_pnl: g(&totals, "realized").unwrap_or_default().to_string(),
            total_volume: g(&totals, "volume").unwrap_or_default().to_string(),
            total_trades: g(&totals, "trades").unwrap_or_default().to_u64(),
            total_fees_paid: g(&totals, "fees").unwrap_or_default().to_string(),
            win_count: wins,
            loss_count: losses,
            win_rate: win_rate(wins, losses),
            largest_win: g(&maxes, "largest_win").map(|v| if v > BigInt::zero() { v } else { BigInt::zero() }).unwrap_or_default().to_string(),
            largest_loss: g(&mins, "largest_loss").map(|v| if v < BigInt::zero() { v } else { BigInt::zero() }).unwrap_or_default().to_string(),
            first_trade_at: g(&mins, "first_trade").map(|v| v.to_u64() as i64).unwrap_or(0),
            last_trade_at: g(&maxes, "last_trade").map(|v| v.to_u64() as i64).unwrap_or(0),
            ..Default::default()
        });
    }
    for (u, t) in pos {
        let qty = positions.get_last(pos_key(&u, &t, "q")).unwrap_or_default();
        let (bq, bc) = bought(&positions, &carry, &u, &t);
        let avg_e6 = price_e6(&bc, &bq);
        let last_e6 = prices.get_last(&t).map(|p| bi(&p)).unwrap_or_default();
        let cost_basis = avg_cost(&qty, &bq, &bc);
        let has_price = prices.has_last(&t);
        let unreal = if has_price { qty.clone() * last_e6.clone() / BigInt::from(PRICE_SCALE) - cost_basis.clone() } else { BigInt::zero() };
        out.positions.push(pnl::Position {
            user_address: u.clone(),
            token_id: t.clone(),
            quantity: qty.to_string(),
            bought_shares: bq.to_string(),
            bought_cost: bc.to_string(),
            avg_entry_price: fmt_price(&avg_e6),
            cost_basis: cost_basis.to_string(),
            realized_pnl: totals.get_last(pos_key(&u, &t, "realized")).unwrap_or_default().to_string(),
            last_price: fmt_price(&last_e6),
            unrealized_pnl: unreal.to_string(),
            ..Default::default()
        });
    }
    Ok(out)
}

#[substreams::handlers::map]
fn map_market_stats(ev: pnl::Events, totals: StoreGetBigInt, prices: StoreGetString) -> Result<pnl::MarketStats, substreams::errors::Error> {
    let mut tokens: BTreeMap<String, String> = BTreeMap::new();
    for f in &ev.fills {
        tokens.insert(f.token_id.clone(), f.exchange.clone());
    }
    let mut out = pnl::MarketStats { block_number: ev.block_number, ..Default::default() };
    for (t, exchange) in tokens {
        out.markets.push(pnl::MarketStat {
            token_id: t.clone(),
            volume: totals.get_last(format!("m:{}:volume", t)).unwrap_or_default().to_string(),
            trades: totals.get_last(format!("m:{}:trades", t)).unwrap_or_default().to_u64(),
            last_price: fmt_price(&prices.get_last(&t).map(|p| bi(&p)).unwrap_or_default()),
            exchange,
            ..Default::default()
        });
    }
    Ok(out)
}

// ---------------------------------------------------------------------------
// Layer 4: SQL
// ---------------------------------------------------------------------------

#[substreams::handlers::map]
fn db_out(
    params: String,
    ev: pnl::Events,
    pnl_updates: pnl::UserPnLUpdates,
    markets: pnl::MarketStats,
) -> Result<DatabaseChanges, substreams::errors::Error> {
    let min_trade = params
        .split('&')
        .find_map(|kv| kv.strip_prefix("min_trade_size="))
        .map(bi)
        .unwrap_or_default();
    let ts = unix_to_timestamp(ev.block_timestamp.seconds);
    let mut tables = Tables::new();

    for f in &ev.fills {
        if bi(&f.collateral) < min_trade { continue; }
        tables
            .create_row("trades", &f.id)
            .set("block_number", f.block_number)
            .set("block_timestamp", &ts)
            .set("tx_hash", &f.tx_hash)
            .set("log_index", f.log_index)
            .set("exchange", &f.exchange)
            .set("position_system", &f.position_system)
            .set("order_hash", &f.order_hash)
            .set("maker", &f.maker)
            .set("taker", &f.taker)
            .set("side", &f.side)
            .set("token_id", &f.token_id)
            .set("shares", &f.shares)
            .set("collateral", &f.collateral)
            .set("fee", &f.fee)
            .set("price", &f.price)
            .set("builder", &f.builder)
            .set("metadata", &f.metadata);
    }
    for x in &ev.fees {
        tables.create_row("fees", &x.id)
            .set("block_number", ev.block_number).set("block_timestamp", &ts)
            .set("tx_hash", &x.tx_hash).set("exchange", &x.exchange)
            .set("receiver", &x.receiver).set("amount", &x.amount);
    }
    for r in &ev.resolutions {
        tables.create_row("resolutions", &r.id)
            .set("block_number", ev.block_number).set("block_timestamp", &ts)
            .set("tx_hash", &r.tx_hash).set("source", &r.source)
            .set("condition_id", &r.condition_id).set("oracle", &r.oracle)
            .set("settled_price", &r.settled_price).set("payouts", r.payouts.join(","));
    }
    for r in &ev.redemptions {
        tables.create_row("redemptions", &r.id)
            .set("block_number", ev.block_number).set("block_timestamp", &ts)
            .set("tx_hash", &r.tx_hash).set("source", &r.source).set("holder", &r.holder)
            .set("condition_id", &r.condition_id).set("position_id", &r.position_id)
            .set("amount", if r.amount.is_empty() { "0" } else { r.amount.as_str() })
            .set("payout", if r.payout.is_empty() { "0" } else { r.payout.as_str() });
    }
    for m in &ev.migrations {
        tables.create_row("migrations", &m.id)
            .set("block_number", ev.block_number).set("block_timestamp", &ts)
            .set("tx_hash", &m.tx_hash).set("holder", &m.holder).set("condition_id", &m.condition_id)
            .set("position_id", &m.position_id).set("outcome_index", &m.outcome_index)
            .set("amount", &m.amount).set("legacy_token_id", &m.legacy_token_id);
    }
    for u in &pnl_updates.users {
        let row = tables.upsert_row("user_pnl", &u.user_address);
        row.set("realized_pnl", &u.realized_pnl)
            .set("total_volume", &u.total_volume)
            .set("total_trades", u.total_trades)
            .set("total_fees_paid", &u.total_fees_paid)
            .set("win_count", u.win_count)
            .set("loss_count", u.loss_count)
            .set("win_rate", &u.win_rate)
            .set("largest_win", &u.largest_win)
            .set("largest_loss", &u.largest_loss)
            .set("last_block", ev.block_number);
        if u.first_trade_at > 0 {
            row.set("first_trade_at", unix_to_timestamp(u.first_trade_at));
        }
        if u.last_trade_at > 0 {
            row.set("last_trade_at", unix_to_timestamp(u.last_trade_at));
        }
    }
    for p in &pnl_updates.positions {
        tables
            .upsert_row("user_positions", [("user_address", p.user_address.clone()), ("token_id", p.token_id.clone())])
            .set("quantity", &p.quantity)
            .set("bought_shares", &p.bought_shares)
            .set("bought_cost", &p.bought_cost)
            .set("avg_entry_price", &p.avg_entry_price)
            .set("cost_basis", &p.cost_basis)
            .set("realized_pnl", &p.realized_pnl)
            .set("last_price", &p.last_price)
            .set("unrealized_pnl", &p.unrealized_pnl)
            .set("last_block", ev.block_number);
    }
    for m in &markets.markets {
        tables
            .upsert_row("markets", &m.token_id)
            .set("exchange", &m.exchange)
            .set("volume", &m.volume)
            .set("trades", m.trades)
            .set("last_price", &m.last_price)
            .set("last_block", ev.block_number);
    }
    Ok(tables.to_database_changes())
}
