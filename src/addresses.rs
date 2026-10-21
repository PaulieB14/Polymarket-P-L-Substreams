//! Polygon (chain id 137) contract addresses, lowercase hex.
//! Source: https://docs.polymarket.com/resources/contracts (fetched 2026-10-21), except the two
//! v1 exchanges, which are the pre-cutover addresses in https://github.com/Polymarket/ctf-exchange.

/// CTF Exchange v1 (legacy CLOB, before the 2026-04-28 cutover).
pub const CTF_EXCHANGE_V1: &str = "0x4bfb41d5b3570defd03c39a9a4d8de6bd8b8982e";
/// Neg Risk CTF Exchange v1 (legacy CLOB).
pub const NEG_RISK_EXCHANGE_V1: &str = "0xc5d563a36ae78145c45a50134d48a1215220f80a";
/// CTF Exchange (current, CTF markets).
pub const CTF_EXCHANGE_V2: &str = "0xe111180000d2663c0091e4f400237545b87b996b";
/// Neg Risk CTF Exchange (current, CTF negative-risk markets).
pub const NEG_RISK_EXCHANGE_V2: &str = "0xe2222d279d744050d28e00520010520000310f59";
/// Polymarket Protocol V2 Exchange ("ExchangeV3 Poly v2"), EIP-712 domain version "3".
pub const EXCHANGE_V3: &str = "0xe3333700ca9d93003f00f0f71f8515005f6c00aa";

/// Conditional Tokens (CTF ERC-1155 ledger).
pub const CONDITIONAL_TOKENS: &str = "0x4d97dcd97ec945f40cf65f87097ace5ea0476045";
/// PositionManager Poly v2 (Protocol V2 ERC-1155 ledger).
pub const POSITION_MANAGER: &str = "0x006f54f7f9a22e0000cc2ab60031000000ae9fef";
pub const ROUTER: &str = "0x12121212006e4cd160d18e3f00711da5c3372600";
pub const BINARY_MODULE: &str = "0x1000008dd9001b968442c1000017eae6e0da00ba";
pub const NEG_RISK_MODULE: &str = "0x200000900045e3b6259600682756002200028933";
pub const COMBINATORIAL_MODULE: &str = "0x30000034706c7d8e12009dab006be20000c031a8";
pub const AUTO_REDEEMER: &str = "0xa1200000d0002264c9a1698e001292d00e1b00af";

/// Neg Risk Adapter (CTF negative-risk approvals).
pub const NEG_RISK_ADAPTER: &str = "0xd91e80cf2e7be2e162c6513ced06f1dd0da35296";
pub const CTF_COLLATERAL_ADAPTER: &str = "0xada100db00ca00073811820692005400218fce1f";
pub const NEG_RISK_CTF_COLLATERAL_ADAPTER: &str = "0xada2005600dec949baf300f4c6120000bdb6eaab";

/// pUSD (CollateralToken Poly v2, proxy) — the current collateral.
pub const PUSD: &str = "0xc011a7e12a19f7b1f670d46f03b03f3342e82dfb";
/// USDC.e — the collateral before pUSD.
pub const USDC_E: &str = "0x2791bca1f2de4661ed88a30c99a7a9449aa84174";

/// UmaCtfAdapter v3.1.0 and v2.0.0.
pub const UMA_ADAPTER_V3_1: &str = "0x157ce2d672854c848c9b79c49a8cc6cc89176a49";
pub const UMA_ADAPTER_V2: &str = "0x6a9d222616c90fca5754cd1333cfd9b7fb6a4f74";

pub const ZERO: &str = "0x0000000000000000000000000000000000000000";

/// Protocol contracts that are never counted as traders or position holders.
/// The last two were already excluded in v1.x and are kept so existing users see no change.
pub const EXCLUDED: [&str; 19] = [
    ZERO, CONDITIONAL_TOKENS, POSITION_MANAGER, CTF_EXCHANGE_V1, NEG_RISK_EXCHANGE_V1,
    CTF_EXCHANGE_V2, NEG_RISK_EXCHANGE_V2, EXCHANGE_V3, ROUTER, BINARY_MODULE, NEG_RISK_MODULE,
    COMBINATORIAL_MODULE, AUTO_REDEEMER, NEG_RISK_ADAPTER, CTF_COLLATERAL_ADAPTER,
    NEG_RISK_CTF_COLLATERAL_ADAPTER, UMA_ADAPTER_V3_1,
    "0x78769d50be1763ed1ca0d5e878d93f05aabff29e",
    "0xa5ef39c3d3e10d0b270233af41cac69796b12966",
];

pub fn is_excluded(addr: &str) -> bool {
    EXCLUDED.iter().any(|e| e.eq_ignore_ascii_case(addr))
}

/// (exchange label, position system) for an exchange address.
pub fn exchange_of(addr: &str) -> Option<(&'static str, &'static str)> {
    match addr {
        CTF_EXCHANGE_V1 => Some(("ctf_v1", "ctf")),
        NEG_RISK_EXCHANGE_V1 => Some(("neg_risk_v1", "ctf")),
        CTF_EXCHANGE_V2 => Some(("ctf_v2", "ctf")),
        NEG_RISK_EXCHANGE_V2 => Some(("neg_risk_v2", "ctf")),
        EXCHANGE_V3 => Some(("exchange_v3", "pm_v2")),
        _ => None,
    }
}
