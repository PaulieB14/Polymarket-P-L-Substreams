//! Hand-written log decoders for the Polymarket contracts this package indexes.
//!
//! Event signatures and field layouts come from Polymarket's own sources:
//! - CTF Exchange v1: github.com/Polymarket/ctf-exchange (src/exchange/mixins/Trading.sol)
//! - CTF Exchange v2 / Neg Risk Exchange v2: github.com/Polymarket/ctf-exchange-v2
//!   (src/exchange/mixins/Trading.sol, abis in polymarket-orderbook-substreams/abis)
//! - Protocol V2 Exchange (ExchangeV3), modules, AutoRedeemer:
//!   github.com/Polymarket/polymarket-v2-external (src/exchange/Exchange.sol,
//!   src/modules/abstract/BaseModule.sol, src/modules/migration/BaseMigrationMixin.sol,
//!   src/utils/AutoRedeemer.sol)
//! - Conditional Tokens (ConditionResolution, PayoutRedemption, PositionSplit,
//!   PositionsMerge, TransferSingle/TransferBatch): abi/profitandloss_contract.abi.json
//! - UmaCtfAdapter v3.1 QuestionResolved: polymarket-v2-external artifacts/UmaCtfAdapterV31.json
//! Topic hashes were computed with keccak256 over the canonical signatures and checked
//! against live Polygon logs (blocks 95,938,000-95,940,000).

use hex_literal::hex;
use substreams::scalar::BigInt;
use substreams::Hex;
use substreams_ethereum::pb::eth::v2::Log;

// --- topic0 values ---------------------------------------------------------
/// OrderFilled(bytes32,address,address,uint256,uint256,uint256,uint256,uint256) — CTF Exchange v1
pub const ORDER_FILLED_V1: [u8; 32] = hex!("d0a08e8c493f9c94f29311604c9de1b4e8c8d4c06bd0c789af57f2d65bfec0f6");
/// OrderFilled(bytes32,address,address,uint8,uint256,uint256,uint256,uint256,bytes32,bytes32) — Exchange v2 and ExchangeV3
pub const ORDER_FILLED_V2: [u8; 32] = hex!("d543adfd945773f1a62f74f0ee55a5e3b9b1a28262980ba90b1a89f2ea84d8ee");
/// FeeCharged(address,uint256) — Exchange v2 and ExchangeV3
pub const FEE_CHARGED: [u8; 32] = hex!("55bb3cade9d43b798a4fe5ffdd05024b2d7870df53920673bfc7e68047cd0ab1");
/// TransferSingle(address,address,address,uint256,uint256) — ERC-1155
pub const TRANSFER_SINGLE: [u8; 32] = hex!("c3d58168c5ae7397731d063d5bbf3d657854427343f4c083240f7aacaa2d0f62");
/// TransferBatch(address,address,address,uint256[],uint256[]) — ERC-1155
pub const TRANSFER_BATCH: [u8; 32] = hex!("4a39dc06d4c0dbc64b70af90fd698a233a518aa5d07e595d983b8c0526c8f7fb");
/// Transfer(address,address,uint256) — ERC-20
pub const ERC20_TRANSFER: [u8; 32] = hex!("ddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef");
/// ConditionResolution(bytes32,address,bytes32,uint256,uint256[]) — Conditional Tokens
pub const CONDITION_RESOLUTION: [u8; 32] = hex!("b44d84d3289691f71497564b85d4233648d9dbae8cbdbb4329f301c3a0185894");
/// PayoutRedemption(address,address,bytes32,bytes32,uint256[],uint256) — Conditional Tokens
pub const PAYOUT_REDEMPTION: [u8; 32] = hex!("2682012a4a4f1973119f1c9b90745d1bd91fa2bab387344f044cb3586864d18d");
/// PositionSplit(address,address,bytes32,bytes32,uint256[],uint256) — Conditional Tokens
pub const CTF_POSITION_SPLIT: [u8; 32] = hex!("2e6bb91f8cbcda0c93623c54d0403a43514fabc40084ec96b6d5379a74786298");
/// PositionsMerge(address,address,bytes32,bytes32,uint256[],uint256) — Conditional Tokens
pub const CTF_POSITIONS_MERGE: [u8; 32] = hex!("6f13ca62553fcc2bcd2372180a43949c1e4cebba603901ede2f4e14f36b282ca");
/// QuestionResolved(bytes32,int256,uint256[]) — UmaCtfAdapter v2.0.0 and v3.1.0
pub const QUESTION_RESOLVED: [u8; 32] = hex!("566c3fbdd12dd86bb341787f6d531f79fd7ad4ce7e3ae2d15ac0ca1b601af9df");
/// ConditionResolved(bytes31,uint256[]) — V2 BinaryModule / NegRiskModule (BaseModule)
pub const CONDITION_RESOLVED: [u8; 32] = hex!("e560c1a2847d4cf2e66de11fe592605556989449f38cc3ab96e2e287daa68caa");
/// PositionRedeemed(address,uint256,address,uint256,uint256) — V2 modules
pub const POSITION_REDEEMED: [u8; 32] = hex!("4fd474b703f46b453536518b4cbc159dfbea6fd663233763d321a9bc8ccdf97f");
/// PositionsSplit(address,bytes31,address,address,uint256) — V2 BinaryModule / NegRiskModule
pub const V2_POSITIONS_SPLIT: [u8; 32] = hex!("b6a6b8b17a43b07b9359dd12d304e42077eede9e672f8646144842c8604582a4");
/// PositionsMerged(address,bytes31,address,uint256) — V2 BinaryModule / NegRiskModule
pub const V2_POSITIONS_MERGED: [u8; 32] = hex!("f233f7be4ce81130a78e8eb2fa626497586c56094d27bfec2e196471e9c60301");
/// PositionMigrated(address,bytes31,uint256,uint256,uint256) — V2 migration mixins
pub const POSITION_MIGRATED: [u8; 32] = hex!("e8d329e97d555e1e9d0da49d3b12d41a966458eee02ff4c869e54c6711d7c80b");
/// Redemption(address,uint256,uint256) — AutoRedeemer (PositionManager balances)
pub const AR_REDEMPTION: [u8; 32] = hex!("eebddeddf4ae1ee54a48517af27958e7666d69c7ba2e3e7c2b0ff87ef5f4491e");
/// BinaryRedemption(address,bytes32,uint256) — AutoRedeemer (CTF binary)
pub const AR_BINARY_REDEMPTION: [u8; 32] = hex!("b434294b5904213c83a167af0068ab82637c6fd4fac945e2abc74ed8d3f4d52a");
/// NegRiskRedemption(address,bytes32,uint256) — AutoRedeemer (CTF negative risk)
pub const AR_NEG_RISK_REDEMPTION: [u8; 32] = hex!("74a51ebefec30281ec6849b727ec7916f9b1a3e5e148d6771d98315215b38b96");

// --- primitive readers ------------------------------------------------------
pub fn word(data: &[u8], i: usize) -> Option<&[u8]> {
    data.get(i * 32..i * 32 + 32)
}
pub fn uint(data: &[u8], i: usize) -> Option<BigInt> {
    word(data, i).map(BigInt::from_unsigned_bytes_be)
}
pub fn topic_addr(log: &Log, i: usize) -> Option<String> {
    log.topics.get(i).filter(|t| t.len() == 32).map(|t| format!("0x{}", Hex(&t[12..32])))
}
pub fn topic_hex(log: &Log, i: usize) -> Option<String> {
    log.topics.get(i).map(|t| format!("0x{}", Hex(t)))
}
pub fn topic_uint(log: &Log, i: usize) -> Option<BigInt> {
    log.topics.get(i).map(|t| BigInt::from_unsigned_bytes_be(t))
}
pub fn word_addr(data: &[u8], i: usize) -> Option<String> {
    word(data, i).map(|w| format!("0x{}", Hex(&w[12..32])))
}
/// Dynamic uint256[] at the ABI head slot `i`.
pub fn uint_array(data: &[u8], i: usize) -> Option<Vec<BigInt>> {
    let off = uint(data, i)?.to_u64() as usize;
    let len_word = data.get(off..off + 32)?;
    let len = BigInt::from_unsigned_bytes_be(len_word).to_u64() as usize;
    if len > 10_000 {
        return None;
    }
    let mut out = Vec::with_capacity(len);
    for k in 0..len {
        let s = off + 32 + k * 32;
        out.push(BigInt::from_unsigned_bytes_be(data.get(s..s + 32)?));
    }
    Some(out)
}
pub fn is(log: &Log, sig: &[u8; 32]) -> bool {
    log.topics.first().map(|t| t.as_slice() == sig).unwrap_or(false)
}

// --- decoded shapes -----------------------------------------------------------
pub struct Fill {
    pub order_hash: String,
    pub maker: String,
    pub taker: String,
    pub buy: bool,
    pub token_id: BigInt,
    pub maker_amount: BigInt,
    pub taker_amount: BigInt,
    pub fee: BigInt,
    pub builder: String,
    pub metadata: String,
    pub v1: bool,
}

impl Fill {
    /// Shares and collateral from the order owner's point of view.
    ///
    /// v1 (ctf-exchange Trading._fillOrder): the owner receives `taking - fee` of the asset
    ///   it takes, so a BUY gets `taker_amount - fee` shares and a SELL gets
    ///   `taker_amount - fee` collateral.
    /// v2 / V3 (ctf-exchange-v2 Trading._settleComplementaryMaker/_settleTakerOrder): the fee is
    ///   always collateral; a BUY pays `maker_amount + fee`, a SELL receives `taker_amount - fee`.
    pub fn shares_and_collateral(&self) -> (BigInt, BigInt) {
        match (self.v1, self.buy) {
            (true, true) => (sat_sub(&self.taker_amount, &self.fee), self.maker_amount.clone()),
            (true, false) => (self.maker_amount.clone(), sat_sub(&self.taker_amount, &self.fee)),
            (false, true) => (self.taker_amount.clone(), self.maker_amount.clone() + self.fee.clone()),
            (false, false) => (self.maker_amount.clone(), sat_sub(&self.taker_amount, &self.fee)),
        }
    }
}

pub fn sat_sub(a: &BigInt, b: &BigInt) -> BigInt {
    if a > b { a.clone() - b.clone() } else { BigInt::zero() }
}

pub fn decode_fill(log: &Log) -> Option<Fill> {
    if log.topics.len() < 4 {
        return None;
    }
    let order_hash = topic_hex(log, 1)?;
    let maker = topic_addr(log, 2)?;
    let taker = topic_addr(log, 3)?;
    if is(log, &ORDER_FILLED_V1) {
        let maker_asset = uint(&log.data, 0)?;
        let taker_asset = uint(&log.data, 1)?;
        let buy = maker_asset.is_zero();
        Some(Fill {
            order_hash, maker, taker, buy,
            token_id: if buy { taker_asset } else { maker_asset },
            maker_amount: uint(&log.data, 2)?,
            taker_amount: uint(&log.data, 3)?,
            fee: uint(&log.data, 4)?,
            builder: String::new(),
            metadata: String::new(),
            v1: true,
        })
    } else if is(log, &ORDER_FILLED_V2) {
        let side = uint(&log.data, 0)?;
        Some(Fill {
            order_hash, maker, taker,
            buy: side.is_zero(),
            token_id: uint(&log.data, 1)?,
            maker_amount: uint(&log.data, 2)?,
            taker_amount: uint(&log.data, 3)?,
            fee: uint(&log.data, 4)?,
            builder: format!("0x{}", Hex(word(&log.data, 5)?)),
            metadata: format!("0x{}", Hex(word(&log.data, 6)?)),
            v1: false,
        })
    } else {
        None
    }
}

/// (operator, from, to, [(id, value)])
pub fn decode_erc1155(log: &Log) -> Option<(String, String, String, Vec<(BigInt, BigInt)>)> {
    let op = topic_addr(log, 1)?;
    let from = topic_addr(log, 2)?;
    let to = topic_addr(log, 3)?;
    if is(log, &TRANSFER_SINGLE) {
        Some((op, from, to, vec![(uint(&log.data, 0)?, uint(&log.data, 1)?)]))
    } else if is(log, &TRANSFER_BATCH) {
        let ids = uint_array(&log.data, 0)?;
        let vals = uint_array(&log.data, 1)?;
        if ids.len() != vals.len() {
            return None;
        }
        Some((op, from, to, ids.into_iter().zip(vals).collect()))
    } else {
        None
    }
}

pub fn decode_erc20_transfer(log: &Log) -> Option<(String, String, BigInt)> {
    if !is(log, &ERC20_TRANSFER) || log.topics.len() < 3 {
        return None;
    }
    Some((topic_addr(log, 1)?, topic_addr(log, 2)?, uint(&log.data, 0)?))
}
