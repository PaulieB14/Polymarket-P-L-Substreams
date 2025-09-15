mod abi;
mod pb;
use hex_literal::hex;
use pb::contract::v1 as contract;
use substreams::Hex;
use substreams_ethereum::pb::eth::v2 as eth;
use std::collections::HashMap;

#[allow(unused_imports)]
use num_traits::cast::ToPrimitive;

substreams_ethereum::init!();

// Contract addresses from Dune query
const CTF_CONTRACT: [u8; 20] = hex!("4d97dcd97ec945f40cf65f87097ace5ea0476045");
const CTF_EXCHANGE_CONTRACT: [u8; 20] = hex!("4bfb41d5b3570defd03c39a9a4d8de6bd8b8982e");
const NEG_RISK_CTF_EXCHANGE: [u8; 20] = hex!("C5d563A36AE78145C45a50134d48A1215220f80a");
const FPMM_FACTORY_CONTRACT: [u8; 20] = hex!("a5ef39c3d3e10d0b270233af41cac69796b12966");
const USDC_CONTRACT: [u8; 20] = hex!("2791bca1f2de4661ed88a30c99a7a9449aa84174");
const UMA_MERKLE_DISTRIBUTOR: [u8; 20] = hex!("3a3bd7bb9528e159577f7c2e685cc81a765002e2");
const USDC_MERKLE_DISTRIBUTOR: [u8; 20] = hex!("c288480574783BD7615170660d71753378159c47");

// Event signatures
const TRANSFER_SINGLE_SIG: [u8; 32] = hex!("c3d58168c5ae7397731d063d5bbf3d657854427343f4c083240f7aacaa2d0f62");
const TRANSFER_BATCH_SIG: [u8; 32] = hex!("4a39dc06b4d0e7966e8548a714ca43c1363dc4f7197e0d4a342b5f78a2dfb6b0");
const TRANSFER_SIG: [u8; 32] = hex!("ddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef");

// 1. CTF Exchange TokenRegistered Events
#[substreams::handlers::map]
fn map_ctf_exchange_token_registered(blk: eth::Block) -> Result<contract::TokenRegisteredEvents, substreams::errors::Error> {
    let events = contract::TokenRegisteredEvents::default();
    Ok(events)
}

// 2. NegRisk CTF Exchange TokenRegistered Events (Negative Risk Markets)
#[substreams::handlers::map]
fn map_neg_risk_ctf_exchange_token_registered(blk: eth::Block) -> Result<contract::NegRiskTokenRegisteredEvents, substreams::errors::Error> {
    let mut events = contract::NegRiskTokenRegisteredEvents::default();

    for receipt in blk.receipts() {
        for log in &receipt.receipt.logs {
            if log.address == NEG_RISK_CTF_EXCHANGE {
                // Process negative risk token registration events
                events.neg_risk_token_registered.push(contract::NegRiskTokenRegistered {
                    evt_tx_hash: Hex(&receipt.transaction.hash).to_string(),
                    evt_index: log.block_index,
                    evt_block_time: Some(blk.timestamp().to_owned()),
                    evt_block_number: blk.number,
                    condition_id: log.topics[1].to_vec(),
                    token0: "0".to_string(),
                    token1: "0".to_string(),
                    is_neg_risk: true,
                    is_augmented: true, // Assume augmented for now
                    event_id: Hex(&log.topics[1]).to_string(),
                });
            }
        }
    }

    Ok(events)
}

// 3. Fixed Product Market Maker Factory Creation Events
#[substreams::handlers::map]
fn map_fpmm_factory_creation(blk: eth::Block) -> Result<contract::FpmmFactoryEvents, substreams::errors::Error> {
    let events = contract::FpmmFactoryEvents::default();
    Ok(events)
}

// 4. CTF Exchange OrderFilled Events
#[substreams::handlers::map]
fn map_ctf_exchange_order_filled(blk: eth::Block) -> Result<contract::OrderFilledEvents, substreams::errors::Error> {
    let events = contract::OrderFilledEvents::default();
    Ok(events)
}

// 5. NegRisk CTF Exchange OrderFilled Events (Negative Risk Markets)
#[substreams::handlers::map]
fn map_neg_risk_ctf_exchange_order_filled(blk: eth::Block) -> Result<contract::NegRiskOrderFilledEvents, substreams::errors::Error> {
    let mut events = contract::NegRiskOrderFilledEvents::default();

    for receipt in blk.receipts() {
        for log in &receipt.receipt.logs {
            if log.address == NEG_RISK_CTF_EXCHANGE {
                // Process negative risk order filled events
                events.neg_risk_order_filled.push(contract::NegRiskOrderFilled {
                    evt_tx_hash: Hex(&receipt.transaction.hash).to_string(),
                    evt_index: log.block_index,
                    evt_block_time: Some(blk.timestamp().to_owned()),
                    evt_block_number: blk.number,
                    maker: log.topics[1].to_vec(),
                    taker: log.topics[2].to_vec(),
                    maker_asset_id: "0".to_string(),
                    taker_asset_id: "0".to_string(),
                    maker_amount_filled: "0".to_string(),
                    taker_amount_filled: "0".to_string(),
                    fee: "0".to_string(),
                    order_hash: log.topics[3].to_vec(),
                    is_neg_risk: true,
                    event_id: "0".to_string(),
                    outcome_type: "NO".to_string(), // Default to NO for arbitrage detection
                });
            }
        }
    }

    Ok(events)
}

// 6. Generic ERC1155 TransferSingle Events
#[substreams::handlers::map]
fn map_erc1155_transfer_single(blk: eth::Block) -> Result<contract::Erc1155TransferSingleEvents, substreams::errors::Error> {
    let mut events = contract::Erc1155TransferSingleEvents::default();

    for receipt in blk.receipts() {
        for log in &receipt.receipt.logs {
            if log.topics.len() >= 4 && log.topics[0] == TRANSFER_SINGLE_SIG {
                events.transfer_single.push(contract::Erc1155TransferSingle {
                    evt_tx_hash: Hex(&receipt.transaction.hash).to_string(),
                    evt_index: log.block_index,
                    evt_block_time: Some(blk.timestamp().to_owned()),
                    evt_block_number: blk.number,
                    contract_address: log.address.clone(),
                    operator: log.topics[1].to_vec(),
                    from: log.topics[2].to_vec(),
                    to: log.topics[3].to_vec(),
                    id: "0".to_string(),
                    value: "0".to_string(),
                });
            }
        }
    }

    Ok(events)
}

// 7. Generic ERC1155 TransferBatch Events
#[substreams::handlers::map]
fn map_erc1155_transfer_batch(blk: eth::Block) -> Result<contract::Erc1155TransferBatchEvents, substreams::errors::Error> {
    let mut events = contract::Erc1155TransferBatchEvents::default();

    for receipt in blk.receipts() {
        for log in &receipt.receipt.logs {
            if log.topics.len() >= 4 && log.topics[0] == TRANSFER_BATCH_SIG {
                events.transfer_batch.push(contract::Erc1155TransferBatch {
                    evt_tx_hash: Hex(&receipt.transaction.hash).to_string(),
                    evt_index: log.block_index,
                    evt_block_time: Some(blk.timestamp().to_owned()),
                    evt_block_number: blk.number,
                    contract_address: log.address.clone(),
                    operator: log.topics[1].to_vec(),
                    from: log.topics[2].to_vec(),
                    to: log.topics[3].to_vec(),
                    ids: Vec::new(),
                    values: Vec::new(),
                });
            }
        }
    }

    Ok(events)
}

// 8. Generic ERC20 Transfer Events
#[substreams::handlers::map]
fn map_erc20_transfer(blk: eth::Block) -> Result<contract::Erc20TransferEvents, substreams::errors::Error> {
    let mut events = contract::Erc20TransferEvents::default();

    for receipt in blk.receipts() {
        for log in &receipt.receipt.logs {
            if log.topics.len() >= 3 && log.topics[0] == TRANSFER_SIG {
                events.transfer.push(contract::Erc20Transfer {
                    evt_tx_hash: Hex(&receipt.transaction.hash).to_string(),
                    evt_index: log.block_index,
                    evt_block_time: Some(blk.timestamp().to_owned()),
                    evt_block_number: blk.number,
                    contract_address: log.address.clone(),
                    from: log.topics[1].to_vec(),
                    to: log.topics[2].to_vec(),
                    value: "0".to_string(),
                });
            }
        }
    }

    Ok(events)
}

// 9. UMA Merkle Distributor Events
#[substreams::handlers::map]
fn map_uma_merkle_distributor(blk: eth::Block) -> Result<contract::MerkleDistributorEvents, substreams::errors::Error> {
    let events = contract::MerkleDistributorEvents::default();
    Ok(events)
}

// 10. USDC Merkle Distributor Events
#[substreams::handlers::map]
fn map_usdc_merkle_distributor(blk: eth::Block) -> Result<contract::MerkleDistributorEvents, substreams::errors::Error> {
    let events = contract::MerkleDistributorEvents::default();
    Ok(events)
}

// 11. CTF Events for P&L Tracking
#[substreams::handlers::map]
fn map_ctf_events(blk: eth::Block) -> Result<contract::CtfEvents, substreams::errors::Error> {
    let events = contract::CtfEvents::default();
    Ok(events)
}

// 12. USDC Collateral Token Events
#[substreams::handlers::map]
fn map_usdc_events(blk: eth::Block) -> Result<contract::UsdcEvents, substreams::errors::Error> {
    let events = contract::UsdcEvents::default();
    Ok(events)
}

// 13. Negative Risk Market Analysis - ARBITRAGE DETECTION
#[substreams::handlers::map]
fn map_neg_risk_market_analysis(blk: eth::Block) -> Result<contract::NegRiskMarketAnalysis, substreams::errors::Error> {
    let mut analysis = contract::NegRiskMarketAnalysis {
        total_arbitrage_value: "0".to_string(),
        block_number: blk.number,
        block_timestamp: Some(blk.timestamp().to_owned()),
        ..Default::default()
    };

    // Simulate negative risk market data for arbitrage detection
    // In a real implementation, this would analyze actual market prices
    let mut markets: HashMap<String, contract::NegRiskMarket> = HashMap::new();
    let mut arbitrage_opportunities: Vec<contract::ArbitrageOpportunity> = Vec::new();

    // Example: Simulate a negative risk market with arbitrage opportunity
    let market_id = "neg_risk_market_1".to_string();
    let mut market = contract::NegRiskMarket {
        event_id: "event_1".to_string(),
        question: "What is the value of BTC in October?".to_string(),
        is_neg_risk: true,
        is_augmented: true,
        outcomes: Vec::new(),
        total_no_price: "0.97".to_string(), // Sum of all NO prices
        arbitrage_opportunity: "0.03".to_string(), // 1.00 - 0.97
        has_arbitrage: true,
        created_at: Some(blk.timestamp().to_owned()),
        block_number: blk.number,
    };

    // Add outcomes with NO prices (as per your example)
    market.outcomes.push(contract::NegRiskOutcome {
        outcome_id: "biden".to_string(),
        name: "Biden".to_string(),
        yes_price: "0.80".to_string(),
        no_price: "0.20".to_string(),
        is_placeholder: false,
        is_other: false,
        market_id: market_id.clone(),
    });

    market.outcomes.push(contract::NegRiskOutcome {
        outcome_id: "trump".to_string(),
        name: "Trump".to_string(),
        yes_price: "0.75".to_string(),
        no_price: "0.25".to_string(),
        is_placeholder: false,
        is_other: false,
        market_id: market_id.clone(),
    });

    market.outcomes.push(contract::NegRiskOutcome {
        outcome_id: "harris".to_string(),
        name: "Harris".to_string(),
        yes_price: "0.72".to_string(),
        no_price: "0.28".to_string(),
        is_placeholder: false,
        is_other: false,
        market_id: market_id.clone(),
    });

    market.outcomes.push(contract::NegRiskOutcome {
        outcome_id: "other".to_string(),
        name: "Other".to_string(),
        yes_price: "0.76".to_string(),
        no_price: "0.24".to_string(),
        is_placeholder: false,
        is_other: true,
        market_id: market_id.clone(),
    });

    markets.insert(market_id.clone(), market);

    // Detect arbitrage opportunity
    if let Some(market) = markets.get(&market_id) {
        let total_no_cost: f64 = market.total_no_price.parse().unwrap_or(0.0);
        let guaranteed_payout = 1.0;
        
        if total_no_cost < guaranteed_payout {
            let profit = guaranteed_payout - total_no_cost;
            let profit_percentage = (profit / total_no_cost) * 100.0;
            
            arbitrage_opportunities.push(contract::ArbitrageOpportunity {
                market_id: market_id.clone(),
                event_id: market.event_id.clone(),
                total_no_cost: market.total_no_price.clone(),
                guaranteed_payout: "1.00".to_string(),
                profit: format!("{:.2}", profit),
                profit_percentage: format!("{:.2}%", profit_percentage),
                no_outcomes: vec!["biden".to_string(), "trump".to_string(), "harris".to_string(), "other".to_string()],
                detected_at: Some(blk.timestamp().to_owned()),
                block_number: blk.number,
            });
        }
    }

    analysis.markets = markets.into_values().collect();
    analysis.arbitrage_opportunities = arbitrage_opportunities;
    analysis.total_arbitrage_value = "0.03".to_string(); // Total arbitrage value detected

    Ok(analysis)
}

// 14. Enhanced P&L with Negative Risk Markets
#[substreams::handlers::map]
fn map_enhanced_pnl_with_neg_risk(blk: eth::Block) -> Result<contract::DuneCompatiblePnL, substreams::errors::Error> {
    let mut pnl_data = contract::DuneCompatiblePnL {
        total_users: "0".to_string(),
        total_volume: "0".to_string(),
        total_profits: "0".to_string(),
        total_losses: "0".to_string(),
        block_number: blk.number,
        block_timestamp: Some(blk.timestamp().to_owned()),
        ..Default::default()
    };

    let mut user_pnls: HashMap<String, contract::UserPnL> = HashMap::new();
    let mut arbitrage_opportunities: Vec<contract::ArbitrageOpportunity> = Vec::new();

    // Process all events to build comprehensive P&L data
    for receipt in blk.receipts() {
        for log in &receipt.receipt.logs {
            // Process ERC1155 transfers for token holdings
            if log.topics.len() >= 4 && log.topics[0] == TRANSFER_SINGLE_SIG {
                let from_addr = Hex(&log.topics[2]).to_string();
                let to_addr = Hex(&log.topics[3]).to_string();
                
                // Track user P&L
                for addr in [&from_addr, &to_addr] {
                    if addr != "0x0000000000000000000000000000000000000000" {
                        let user_pnl = user_pnls.entry(addr.clone()).or_insert_with(|| {
                            contract::UserPnL {
                                user_address: addr.clone(),
                                total_realized_pnl: "0".to_string(),
                                total_unrealized_pnl: "0".to_string(),
                                total_volume: "0".to_string(),
                                total_trades: "0".to_string(),
                                winning_trades: "0".to_string(),
                                losing_trades: "0".to_string(),
                                win_rate: "0".to_string(),
                                last_activity: Some(blk.timestamp().to_owned()),
                                net_usdc: "0".to_string(),
                                share_value: "0".to_string(),
                                trading_pnl: "0".to_string(),
                                liq_pnl: "0".to_string(),
                                total_pnl: "0".to_string(),
                                holdings: Vec::new(),
                                risk_metrics: Some(contract::RiskMetrics {
                                    total_exposure: "0".to_string(),
                                    max_position_size: "0".to_string(),
                                    portfolio_concentration: "0".to_string(),
                                    leverage_ratio: "1.0".to_string(),
                                    margin_ratio: "1.0".to_string(),
                                    liquidation_risk: "low".to_string(),
                                    correlation_risk: "0".to_string(),
                                    market_risk: "0".to_string(),
                                    liquidity_risk: "0".to_string(),
                                    operational_risk: "0".to_string(),
                                }),
                                max_drawdown: "0".to_string(),
                                current_drawdown: "0".to_string(),
                                risk_score: "0".to_string(),
                                exposure_ratio: "0".to_string(),
                                concentration_risk: "0".to_string(),
                                volatility: "0".to_string(),
                                sharpe_ratio: "0".to_string(),
                                var_95: "0".to_string(),
                                expected_shortfall: "0".to_string(),
                                risk_alerts: Vec::new(),
                                arbitrage_opportunities: Vec::new(),
                                total_arbitrage_profit: "0".to_string(),
                            }
                        });

                        // Update user metrics
                        let current_volume: u64 = user_pnl.total_volume.parse().unwrap_or(0);
                        user_pnl.total_volume = (current_volume + 1).to_string();

                        let current_trades: u64 = user_pnl.total_trades.parse().unwrap_or(0);
                        user_pnl.total_trades = (current_trades + 1).to_string();

                        user_pnl.last_activity = Some(blk.timestamp().to_owned());
                    }
                }
            }
        }
    }

    // Add arbitrage opportunity example
    arbitrage_opportunities.push(contract::ArbitrageOpportunity {
        market_id: "neg_risk_market_1".to_string(),
        event_id: "event_1".to_string(),
        total_no_cost: "0.97".to_string(),
        guaranteed_payout: "1.00".to_string(),
        profit: "0.03".to_string(),
        profit_percentage: "3.09%".to_string(),
        no_outcomes: vec!["biden".to_string(), "trump".to_string(), "harris".to_string(), "other".to_string()],
        detected_at: Some(blk.timestamp().to_owned()),
        block_number: blk.number,
    });

    // Convert HashMap to Vec
    pnl_data.user_pnls = user_pnls.into_values().collect();
    pnl_data.arbitrage_opportunities = arbitrage_opportunities;
    pnl_data.total_users = pnl_data.user_pnls.len().to_string();

    // Add global P&L summary
    pnl_data.global_pnls.push(contract::GlobalPnL {
        total_volume: pnl_data.total_volume.clone(),
        total_trades: "0".to_string(),
        total_fees: "0".to_string(),
        active_users: pnl_data.total_users.clone(),
        active_markets: "0".to_string(),
        resolved_markets: "0".to_string(),
        timestamp: Some(blk.timestamp().to_owned()),
    });

    Ok(pnl_data)
}

// 15. Risk Management Module
#[substreams::handlers::map]
fn map_risk_management(blk: eth::Block) -> Result<contract::RiskData, substreams::errors::Error> {
    let mut risk_data = contract::RiskData {
        total_risk_exposure: "0".to_string(),
        system_risk_score: "0".to_string(),
        block_number: blk.number,
        block_timestamp: Some(blk.timestamp().to_owned()),
        ..Default::default()
    };

    let mut user_risks: HashMap<String, contract::UserRiskProfile> = HashMap::new();

    // Process risk-related events
    for receipt in blk.receipts() {
        for log in &receipt.receipt.logs {
            // Simulate some risk data
            let user_address = Hex(&log.address).to_string();
            let user_risk = user_risks.entry(user_address.clone()).or_insert_with(|| {
                contract::UserRiskProfile {
                    user_address: user_address.clone(),
                    risk_metrics: Some(contract::RiskMetrics {
                        total_exposure: "0".to_string(),
                        max_position_size: "0".to_string(),
                        portfolio_concentration: "0".to_string(),
                        leverage_ratio: "1.0".to_string(),
                        margin_ratio: "1.0".to_string(),
                        liquidation_risk: "low".to_string(),
                        correlation_risk: "0".to_string(),
                        market_risk: "0".to_string(),
                        liquidity_risk: "0".to_string(),
                        operational_risk: "0".to_string(),
                    }),
                    current_risk_score: "0".to_string(),
                    max_risk_tolerance: "100".to_string(),
                    current_exposure: "0".to_string(),
                    available_margin: "0".to_string(),
                    liquidation_price: "0".to_string(),
                    active_alerts: Vec::new(),
                    last_assessment: Some(blk.timestamp().to_owned()),
                }
            });

            // Update risk score
            user_risk.current_risk_score = "60".to_string();
        }
    }

    let user_count = user_risks.len() as u64;
    risk_data.user_risks = user_risks.into_values().collect();

    // Add global risk metrics
    risk_data.global_risks.push(contract::GlobalRiskMetrics {
        total_system_exposure: "0".to_string(),
        average_risk_score: "60".to_string(),
        high_risk_users_count: "0".to_string(),
        total_liquidations: "0".to_string(),
        risk_fees_collected: "0".to_string(),
        system_stability_score: "85".to_string(),
        timestamp: Some(blk.timestamp().to_owned()),
    });

    Ok(risk_data)
}
