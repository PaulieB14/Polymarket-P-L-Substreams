//! ABI decoders for Polymarket contract events

use substreams::Hex;
use substreams_ethereum::pb::eth::v2::Log;

/// Decoded OrderFilled event
///
/// Used for both CLOB v1 and v2 fills. For v2, the original event has a
/// single `tokenId` and a `side` (BUY=0/SELL=1) instead of `makerAssetId`
/// and `takerAssetId`. We synthesize the v1-shape fields from `(side, tokenId)`
/// so the existing P&L pipeline (which keys on maker/taker asset == "0") keeps
/// working unchanged.
pub struct OrderFilledEvent {
    pub order_hash: String,
    pub maker: Vec<u8>,
    pub taker: Vec<u8>,
    pub maker_asset_id: String,
    pub taker_asset_id: String,
    pub maker_amount_filled: String,
    pub taker_amount_filled: String,
    pub fee: String,
    /// "v1" or "v2" — identifies which exchange generation emitted this fill.
    pub exchange_version: &'static str,
    /// V2-only: bytes32 builder attribution (hex). Empty for v1.
    pub builder: String,
    /// V2-only: bytes32 order metadata (hex). Empty for v1.
    pub metadata: String,
}

/// Decoded ERC1155 TransferSingle event
pub struct TransferSingleEvent {
    pub operator: Vec<u8>,
    pub from: Vec<u8>,
    pub to: Vec<u8>,
    pub token_id: String,
    pub amount: String,
}

/// Decoded ERC20 Transfer event
pub struct TransferEvent {
    pub from: Vec<u8>,
    pub to: Vec<u8>,
    pub amount: String,
}

/// V1 OrderFilled signature: OrderFilled(bytes32,address,address,uint256,uint256,uint256,uint256,uint256)
const ORDER_FILLED_SIG: [u8; 32] = [
    0xd0, 0xa0, 0x8e, 0x8c, 0x49, 0x3f, 0x9c, 0x94, 0xf2, 0x9c, 0xd8, 0x23,
    0xd8, 0x49, 0x1c, 0x59, 0x5b, 0xa2, 0x16, 0x41, 0x3f, 0x5c, 0x5a, 0xf0,
    0xab, 0x29, 0x66, 0x2a, 0x79, 0x5b, 0x4b, 0xa4,
];

/// V2 OrderFilled signature: OrderFilled(bytes32,address,address,uint8,uint256,uint256,uint256,uint256,bytes32,bytes32)
/// keccak256("OrderFilled(bytes32,address,address,uint8,uint256,uint256,uint256,uint256,bytes32,bytes32)")
pub const ORDER_FILLED_V2_SIG: [u8; 32] = [
    0xd5, 0x43, 0xad, 0xfd, 0x94, 0x57, 0x73, 0xf1, 0xa6, 0x2f, 0x74, 0xf0,
    0xee, 0x55, 0xa5, 0xe3, 0xb9, 0xb1, 0xa2, 0x82, 0x62, 0x98, 0x0b, 0xa9,
    0x0b, 0x1a, 0x89, 0xf2, 0xea, 0x84, 0xd8, 0xee,
];

/// Decode V1 OrderFilled event from log.
///
/// Note: the legacy hand-rolled decoder reads all fields from `log.data`, matching
/// how the deployed v1 contracts emit. Preserved verbatim for backward compatibility.
pub fn decode_order_filled(log: &Log) -> Option<OrderFilledEvent> {
    if log.topics.is_empty() {
        return None;
    }

    if log.data.len() < 224 {
        return None;
    }

    let order_hash = Hex(&log.data[0..32]).to_string();
    let maker = log.data[44..64].to_vec();
    let taker = log.data[76..96].to_vec();

    let maker_asset_id = parse_uint256(&log.data[96..128]);
    let taker_asset_id = parse_uint256(&log.data[128..160]);
    let maker_amount_filled = parse_uint256(&log.data[160..192]);
    let taker_amount_filled = parse_uint256(&log.data[192..224]);

    let fee = if log.data.len() >= 256 {
        parse_uint256(&log.data[224..256])
    } else {
        "0".to_string()
    };

    Some(OrderFilledEvent {
        order_hash,
        maker,
        taker,
        maker_asset_id,
        taker_asset_id,
        maker_amount_filled,
        taker_amount_filled,
        fee,
        exchange_version: "v1",
        builder: String::new(),
        metadata: String::new(),
    })
}

/// Decode V2 OrderFilled event from log.
///
/// V2 indexed fields live in topics; the rest is in data:
///   topics[0] = event signature
///   topics[1] = orderHash (bytes32)
///   topics[2] = maker (address, last 20 bytes of 32)
///   topics[3] = taker (address, last 20 bytes of 32)
///   data: side(uint8 → 32) | tokenId(32) | makerAmountFilled(32) | takerAmountFilled(32)
///         | fee(32) | builder(bytes32) | metadata(bytes32)  = 224 bytes
///
/// We synthesize v1-style maker_asset_id/taker_asset_id from (side, tokenId) so the
/// existing P&L logic that branches on `maker_asset_id == "0"` keeps working.
pub fn decode_order_filled_v2(log: &Log) -> Option<OrderFilledEvent> {
    if log.topics.len() < 4 {
        return None;
    }
    if log.topics[0] != ORDER_FILLED_V2_SIG {
        return None;
    }
    if log.data.len() < 224 {
        return None;
    }

    let order_hash = Hex(&log.topics[1]).to_string();
    let maker = log.topics[2][12..32].to_vec();
    let taker = log.topics[3][12..32].to_vec();

    // side is uint8 right-aligned in a 32-byte slot
    let side = log.data[31];
    let token_id = parse_uint256(&log.data[32..64]);
    let maker_amount_filled = parse_uint256(&log.data[64..96]);
    let taker_amount_filled = parse_uint256(&log.data[96..128]);
    let fee = parse_uint256(&log.data[128..160]);
    let builder = Hex(&log.data[160..192]).to_string();
    let metadata = Hex(&log.data[192..224]).to_string();

    // Map (side, token_id) → v1-shape (maker_asset_id, taker_asset_id):
    //   side=0 (BUY):  maker pays USDC, receives token  → maker="0", taker=token_id
    //   side=1 (SELL): maker pays token, receives USDC  → maker=token_id, taker="0"
    let (maker_asset_id, taker_asset_id) = match side {
        0 => ("0".to_string(), token_id),
        1 => (token_id, "0".to_string()),
        _ => (token_id, "0".to_string()),
    };

    Some(OrderFilledEvent {
        order_hash,
        maker,
        taker,
        maker_asset_id,
        taker_asset_id,
        maker_amount_filled,
        taker_amount_filled,
        fee,
        exchange_version: "v2",
        builder,
        metadata,
    })
}

/// Decode ERC1155 TransferSingle event
/// Event: TransferSingle(address indexed operator, address indexed from, address indexed to, uint256 id, uint256 value)
pub fn decode_erc1155_transfer_single(log: &Log) -> Option<TransferSingleEvent> {
    if log.topics.len() < 4 || log.data.len() < 64 {
        return None;
    }

    let operator = log.topics[1][12..32].to_vec();
    let from = log.topics[2][12..32].to_vec();
    let to = log.topics[3][12..32].to_vec();

    let token_id = parse_uint256(&log.data[0..32]);
    let amount = parse_uint256(&log.data[32..64]);

    Some(TransferSingleEvent {
        operator,
        from,
        to,
        token_id,
        amount,
    })
}

/// Decode ERC20 Transfer event
/// Event: Transfer(address indexed from, address indexed to, uint256 value)
pub fn decode_erc20_transfer(log: &Log) -> Option<TransferEvent> {
    if log.topics.len() < 3 || log.data.len() < 32 {
        return None;
    }

    let from = log.topics[1][12..32].to_vec();
    let to = log.topics[2][12..32].to_vec();
    let amount = parse_uint256(&log.data[0..32]);

    Some(TransferEvent { from, to, amount })
}

/// Parse uint256 from bytes (big-endian)
fn parse_uint256(data: &[u8]) -> String {
    if data.len() != 32 {
        return "0".to_string();
    }

    // Skip leading zeros and convert to decimal string
    let result = num_bigint::BigUint::from_bytes_be(data);
    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_uint256() {
        let data = [0u8; 32];
        assert_eq!(parse_uint256(&data), "0");

        let mut data = [0u8; 32];
        data[31] = 1;
        assert_eq!(parse_uint256(&data), "1");

        let mut data = [0u8; 32];
        data[31] = 100;
        assert_eq!(parse_uint256(&data), "100");
    }
}
