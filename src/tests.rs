use super::*;

#[test]
fn price_formatting() {
    assert_eq!(fmt_price(&price_e6(&bi("520000"), &bi("1000000"))), "0.520000");
    assert_eq!(fmt_price(&bi("-1500000")), "-1.500000");
    assert_eq!(fmt_price(&price_e6(&bi("1"), &bi("0"))), "0.000000");
}

#[test]
fn average_cost_and_realized() {
    // buy 100 @0.40 then 100 @0.60 -> avg 0.50; sell 50 for 35 -> realized 35 - 25 = 10
    let bq = bi("200000000");
    let bc = bi("100000000");
    let cost = avg_cost(&bi("50000000"), &bq, &bc);
    assert_eq!(cost.to_string(), "25000000");
    assert_eq!((bi("35000000") - cost).to_string(), "10000000");
    assert_eq!(avg_cost(&bi("5"), &bi("0"), &bi("0")).to_string(), "0");
}

#[test]
fn win_rate_format() {
    assert_eq!(win_rate(0, 0), "0.0000");
    assert_eq!(win_rate(2, 1), "0.6666");
    assert_eq!(win_rate(3, 0), "1.0000");
}

#[test]
fn share_allocation() {
    assert_eq!(share_of(&bi("90"), &bi("1"), &bi("3")).to_string(), "30");
    assert_eq!(share_of(&bi("90"), &bi("1"), &bi("0")).to_string(), "0");
}

#[test]
fn timestamp_format() {
    assert_eq!(unix_to_timestamp(0), "1970-01-01 00:00:00");
    assert_eq!(unix_to_timestamp(1_777_334_400), "2026-04-28 00:00:00");
}

#[test]
fn excluded_and_exchanges() {
    assert!(addresses::is_excluded("0xE3333700cA9d93003F00f0F71f8515005F6c00Aa"));
    assert!(!addresses::is_excluded("0x1111111111111111111111111111111111111111"));
    assert_eq!(addresses::exchange_of(addresses::EXCHANGE_V3), Some(("exchange_v3", "pm_v2")));
    for a in addresses::EXCLUDED {
        assert_eq!(a, a.to_lowercase());
    }
}

fn w(v: u64) -> Vec<u8> {
    let mut b = vec![0u8; 32];
    b[24..].copy_from_slice(&v.to_be_bytes());
    b
}
fn a(last: u8) -> Vec<u8> {
    let mut b = vec![0u8; 32];
    b[31] = last;
    b
}
fn log(topics: Vec<Vec<u8>>, data: Vec<u8>) -> eth::Log {
    eth::Log { topics, data, ..Default::default() }
}

#[test]
fn v2_fill_buy_pays_fee_on_top() {
    // OrderFilled v2: side=BUY(0), tokenId=7, makerAmount=600 collateral, takerAmount=1000 shares, fee=6
    let data = [w(0), w(7), w(600), w(1000), w(6), vec![0; 32], vec![0; 32]].concat();
    let l = log(vec![abi::ORDER_FILLED_V2.to_vec(), a(1), a(2), a(3)], data);
    let f = abi::decode_fill(&l).unwrap();
    assert!(f.buy && !f.v1);
    let (shares, coll) = f.shares_and_collateral();
    assert_eq!((shares.to_string(), coll.to_string()), ("1000".into(), "606".into()));
    assert_eq!(f.maker, "0x0000000000000000000000000000000000000002");
}

#[test]
fn v2_fill_sell_receives_net_of_fee() {
    let data = [w(1), w(7), w(1000), w(600), w(6), vec![0; 32], vec![0; 32]].concat();
    let l = log(vec![abi::ORDER_FILLED_V2.to_vec(), a(1), a(2), a(3)], data);
    let (shares, coll) = abi::decode_fill(&l).unwrap().shares_and_collateral();
    assert_eq!((shares.to_string(), coll.to_string()), ("1000".into(), "594".into()));
}

#[test]
fn v1_fill_buy_fee_in_shares() {
    // v1: makerAssetId=0 (collateral) -> BUY of takerAssetId=9; owner receives taking - fee shares
    let data = [w(0), w(9), w(500), w(1000), w(10)].concat();
    let l = log(vec![abi::ORDER_FILLED_V1.to_vec(), a(1), a(2), a(3)], data);
    let f = abi::decode_fill(&l).unwrap();
    assert!(f.buy && f.v1);
    assert_eq!(f.token_id.to_string(), "9");
    let (shares, coll) = f.shares_and_collateral();
    assert_eq!((shares.to_string(), coll.to_string()), ("990".into(), "500".into()));
}

#[test]
fn transfer_batch_is_flattened() {
    // TransferBatch(operator, from, to, uint256[] ids, uint256[] values)
    let data = [w(64), w(160), w(2), w(11), w(12), w(2), w(100), w(200)].concat();
    let l = log(vec![abi::TRANSFER_BATCH.to_vec(), a(1), a(2), a(3)], data);
    let (_, _, _, items) = abi::decode_erc1155(&l).unwrap();
    let got: Vec<(String, String)> = items.iter().map(|(i, v)| (i.to_string(), v.to_string())).collect();
    assert_eq!(got, vec![("11".into(), "100".into()), ("12".into(), "200".into())]);
}

#[test]
fn non_fill_log_is_ignored() {
    let l = log(vec![abi::FEE_CHARGED.to_vec(), a(1), a(2), a(3)], w(1));
    assert!(abi::decode_fill(&l).is_none());
}
