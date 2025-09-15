mod abi;
mod pb;
use hex_literal::hex;
use pb::contract::v1 as contract;
use substreams::Hex;
use substreams_ethereum::pb::eth::v2 as eth;
use substreams_ethereum::Event;
use std::collections::HashMap;

#[allow(unused_imports)]
use num_traits::cast::ToPrimitive;

substreams_ethereum::init!();

// Contract addresses
const CTF_CONTRACT: [u8; 20] = hex!("4d97dcd97ec945f40cf65f87097ace5ea0476045");
const CTF_EXCHANGE_CONTRACT: [u8; 20] = hex!("4bfb41d5b3570defd03c39a9a4d8de6bd8b8982e");
const USDC_CONTRACT: [u8; 20] = hex!("2791bca1f2de4661ed88a30c99a7a9449aa84174");

// Negative Risk Management Contracts
const NEG_RISK_EXCHANGE: [u8; 20] = hex!("C5d563A36AE78145C45a50134d48A1215220f80a");
const NEG_RISK_ADAPTER: [u8; 20] = hex!("d91E80cF2E7be2e162c6513ceD06f1dD0dA35296");
const NEG_RISK_OPERATOR: [u8; 20] = hex!("71523d0f655B41E805Cec45b17163f528B59B820");
const NEG_RISK_WRAPPED_COLLATERAL: [u8; 20] = hex!("3A3BD7bb9528E159577F7C2e685CC81A765002E2");

// CTF Events Handler
#[substreams::handlers::map]
fn map_ctf_events(blk: eth::Block) -> Result<contract::CtfEvents, substreams::errors::Error> {
    let mut events = contract::CtfEvents::default();

    for receipt in blk.receipts() {
        for log in &receipt.receipt.logs {
            if log.address == CTF_CONTRACT {
                // Condition Preparation
                if let Some(event) = abi::profitandloss_contract::events::ConditionPreparation::match_and_decode(log) {
                    events.condition_preparations.push(contract::CtfConditionPreparation {
                        evt_tx_hash: Hex(&receipt.transaction.hash).to_string(),
                        evt_index: log.block_index,
                        evt_block_time: Some(blk.timestamp().to_owned()),
                        evt_block_number: blk.number,
                        condition_id: Vec::from(event.condition_id),
                        oracle: event.oracle,
                        question_id: Vec::from(event.question_id),
                        outcome_slot_count: event.outcome_slot_count.to_i32() as u32,
                    });
                }

                // Condition Resolution
                if let Some(event) = abi::profitandloss_contract::events::ConditionResolution::match_and_decode(log) {
                    events.condition_resolutions.push(contract::CtfConditionResolution {
                        evt_tx_hash: Hex(&receipt.transaction.hash).to_string(),
                        evt_index: log.block_index,
                        evt_block_time: Some(blk.timestamp().to_owned()),
                        evt_block_number: blk.number,
                        condition_id: Vec::from(event.condition_id),
                        oracle: event.oracle,
                        question_id: Vec::from(event.question_id),
                        outcome_slot_count: event.outcome_slot_count.to_i32() as u32,
                        payout_numerators: event.payout_numerators.into_iter().map(|x| x.to_string()).collect(),
                        payout_denominator: "1".to_string(), // Default for now
                    });
                }

                // Position Split
                if let Some(event) = abi::profitandloss_contract::events::PositionSplit::match_and_decode(log) {
                    events.position_splits.push(contract::CtfPositionSplit {
                        evt_tx_hash: Hex(&receipt.transaction.hash).to_string(),
                        evt_index: log.block_index,
                        evt_block_time: Some(blk.timestamp().to_owned()),
                        evt_block_number: blk.number,
                        stakeholder: event.stakeholder,
                        collateral_token: event.collateral_token,
                        parent_collection_id: Vec::from(event.parent_collection_id),
                        condition_id: Vec::from(event.condition_id),
                        partition: event.partition.into_iter().map(|x| x.to_string()).collect(),
                        amount: event.amount.to_string(),
                    });
                }

                // Position Merge
                if let Some(event) = abi::profitandloss_contract::events::PositionsMerge::match_and_decode(log) {
                    events.position_merges.push(contract::CtfPositionMerge {
                        evt_tx_hash: Hex(&receipt.transaction.hash).to_string(),
                        evt_index: log.block_index,
                        evt_block_time: Some(blk.timestamp().to_owned()),
                        evt_block_number: blk.number,
                        stakeholder: event.stakeholder,
                        collateral_token: event.collateral_token,
                        parent_collection_id: Vec::from(event.parent_collection_id),
                        condition_id: Vec::from(event.condition_id),
                        partition: event.partition.into_iter().map(|x| x.to_string()).collect(),
                        amount: event.amount.to_string(),
                    });
                }

                // Position Redeem
                if let Some(event) = abi::profitandloss_contract::events::PayoutRedemption::match_and_decode(log) {
                    events.position_redemptions.push(contract::CtfPositionRedeem {
                        evt_tx_hash: Hex(&receipt.transaction.hash).to_string(),
                        evt_index: log.block_index,
                        evt_block_time: Some(blk.timestamp().to_owned()),
                        evt_block_number: blk.number,
                        redeemer: event.redeemer,
                        collateral_token: event.collateral_token,
                        parent_collection_id: Vec::from(event.parent_collection_id),
                        condition_id: Vec::from(event.condition_id),
                        index_sets: event.index_sets.into_iter().map(|x| x.to_string()).collect(),
                        payout: event.payout.to_string(),
                    });
                }

                // Transfer Single
                if let Some(event) = abi::profitandloss_contract::events::TransferSingle::match_and_decode(log) {
                    events.transfer_singles.push(contract::CtfTransferSingle {
                        evt_tx_hash: Hex(&receipt.transaction.hash).to_string(),
                        evt_index: log.block_index,
                        evt_block_time: Some(blk.timestamp().to_owned()),
                        evt_block_number: blk.number,
                        operator: event.operator,
                        from: event.from,
                        to: event.to,
                        token_id: event.id.to_string(),
                        value: event.value.to_string(),
                    });
                }

                // Transfer Batch
                if let Some(event) = abi::profitandloss_contract::events::TransferBatch::match_and_decode(log) {
                    events.transfer_batches.push(contract::CtfTransferBatch {
                        evt_tx_hash: Hex(&receipt.transaction.hash).to_string(),
                        evt_index: log.block_index,
                        evt_block_time: Some(blk.timestamp().to_owned()),
                        evt_block_number: blk.number,
                        operator: event.operator,
                        from: event.from,
                        to: event.to,
                        token_ids: event.ids.into_iter().map(|x| x.to_string()).collect(),
                        values: event.values.into_iter().map(|x| x.to_string()).collect(),
                    });
                }
            }
        }
    }

    Ok(events)
}

// CTF Exchange Events Handler
#[substreams::handlers::map]
fn map_ctf_exchange_events(blk: eth::Block) -> Result<contract::CtfExchangeEvents, substreams::errors::Error> {
    let events = contract::CtfExchangeEvents::default();

    for receipt in blk.receipts() {
        for log in &receipt.receipt.logs {
            if log.address == CTF_EXCHANGE_CONTRACT {
                // Note: These would need the CTF Exchange ABI to be properly implemented
                // For now, we'll create placeholder handlers
                // In a real implementation, you'd need to add the CTF Exchange ABI
            }
        }
    }

    Ok(events)
}

// USDC Events Handler
#[substreams::handlers::map]
fn map_usdc_events(blk: eth::Block) -> Result<contract::UsdcEvents, substreams::errors::Error> {
    let events = contract::UsdcEvents::default();

    for receipt in blk.receipts() {
        for log in &receipt.receipt.logs {
            if log.address == USDC_CONTRACT {
                // Note: These would need the USDC ABI to be properly implemented
                // For now, we'll create placeholder handlers
                // In a real implementation, you'd need to add the USDC ABI
            }
        }
    }

    Ok(events)
}

// User Position Tracking Handler
#[substreams::handlers::map]
fn map_user_positions(blk: eth::Block) -> Result<contract::UserPositions, substreams::errors::Error> {
    let mut positions = contract::UserPositions::default();
    let mut user_positions: HashMap<String, contract::UserPosition> = HashMap::new();
    let mut position_updates: Vec<contract::PositionUpdate> = Vec::new();
    let mut market_resolutions: Vec<contract::MarketResolution> = Vec::new();

    // Track position changes from CTF events
    for receipt in blk.receipts() {
        for log in &receipt.receipt.logs {
            if log.address == CTF_CONTRACT {
                // Process TransferSingle events for position tracking
                if let Some(event) = abi::profitandloss_contract::events::TransferSingle::match_and_decode(log) {
                    let from_addr = Hex(&event.from).to_string();
                    let to_addr = Hex(&event.to).to_string();
                    let token_id = event.id.to_string();
                    let value = event.value.to_string();

                    // Skip zero transfers
                    if value == "0" {
                        continue;
                    }

                    // Update position for sender (decrease)
                    if from_addr != "0x0000000000000000000000000000000000000000" {
                        let position_key = format!("{}:{}", from_addr, token_id);
                        let position = user_positions.entry(position_key.clone()).or_insert_with(|| {
                            contract::UserPosition {
                                user_address: from_addr.clone(),
                                token_id: token_id.clone(),
                                condition_id: "".to_string(),
                                outcome_index: "0".to_string(),
                                amount_held: "0".to_string(),
                                average_price: "0".to_string(),
                                total_bought: "0".to_string(),
                                total_sold: "0".to_string(),
                                realized_pnl: "0".to_string(),
                                unrealized_pnl: "0".to_string(),
                                first_seen: Some(blk.timestamp().to_owned()),
                                last_updated: Some(blk.timestamp().to_owned()),
                            }
                        });

                        // Decrease position
                        let current_amount: u64 = position.amount_held.parse().unwrap_or(0);
                        let transfer_amount: u64 = value.parse().unwrap_or(0);
                        let new_amount = if current_amount >= transfer_amount {
                            current_amount - transfer_amount
                        } else {
                            0
                        };
                        position.amount_held = new_amount.to_string();
                        position.last_updated = Some(blk.timestamp().to_owned());

                        // Track as sell
                        let current_sold: u64 = position.total_sold.parse().unwrap_or(0);
                        position.total_sold = (current_sold + transfer_amount).to_string();

                        // Record position update
                        position_updates.push(contract::PositionUpdate {
                            user_address: from_addr,
                            token_id: token_id.clone(),
                            transaction_hash: Hex(&receipt.transaction.hash).to_string(),
                            update_type: "sell".to_string(),
                            amount: value.clone(),
                            price: "0".to_string(), // Would need price calculation
                            pnl_change: "0".to_string(), // Would need P&L calculation
                            timestamp: Some(blk.timestamp().to_owned()),
                            block_number: blk.number,
                        });
                    }

                    // Update position for receiver (increase)
                    if to_addr != "0x0000000000000000000000000000000000000000" {
                        let position_key = format!("{}:{}", to_addr, token_id);
                        let position = user_positions.entry(position_key.clone()).or_insert_with(|| {
                            contract::UserPosition {
                                user_address: to_addr.clone(),
                                token_id: token_id.clone(),
                                condition_id: "".to_string(),
                                outcome_index: "0".to_string(),
                                amount_held: "0".to_string(),
                                average_price: "0".to_string(),
                                total_bought: "0".to_string(),
                                total_sold: "0".to_string(),
                                realized_pnl: "0".to_string(),
                                unrealized_pnl: "0".to_string(),
                                first_seen: Some(blk.timestamp().to_owned()),
                                last_updated: Some(blk.timestamp().to_owned()),
                            }
                        });

                        // Increase position
                        let current_amount: u64 = position.amount_held.parse().unwrap_or(0);
                        let transfer_amount: u64 = value.parse().unwrap_or(0);
                        position.amount_held = (current_amount + transfer_amount).to_string();
                        position.last_updated = Some(blk.timestamp().to_owned());

                        // Track as buy
                        let current_bought: u64 = position.total_bought.parse().unwrap_or(0);
                        position.total_bought = (current_bought + transfer_amount).to_string();

                        // Record position update
                        position_updates.push(contract::PositionUpdate {
                            user_address: to_addr,
                            token_id: token_id.clone(),
                            transaction_hash: Hex(&receipt.transaction.hash).to_string(),
                            update_type: "buy".to_string(),
                            amount: value.clone(),
                            price: "0".to_string(), // Would need price calculation
                            pnl_change: "0".to_string(), // Would need P&L calculation
                            timestamp: Some(blk.timestamp().to_owned()),
                            block_number: blk.number,
                        });
                    }
                }

                // Process Condition Resolution for market resolutions
                if let Some(event) = abi::profitandloss_contract::events::ConditionResolution::match_and_decode(log) {
                    market_resolutions.push(contract::MarketResolution {
                        condition_id: Hex(&event.condition_id).to_string(),
                        question_id: Hex(&event.question_id).to_string(),
                        payout_numerators: event.payout_numerators.into_iter().map(|x| x.to_string()).collect(),
                        payout_denominator: "1".to_string(),
                        winning_outcome: "0".to_string(), // Would need to determine winning outcome
                        resolution_timestamp: Some(blk.timestamp().to_owned()),
                        block_number: blk.number,
                    });
                }
            }
        }
    }

    // Convert HashMap to Vec
    positions.positions = user_positions.into_values().collect();
    positions.updates = position_updates;
    positions.resolutions = market_resolutions;

    Ok(positions)
}

// P&L Data Aggregation Handler
#[substreams::handlers::map]
fn map_pnl_data(blk: eth::Block) -> Result<contract::PnLData, substreams::errors::Error> {
    let mut pnl_data = contract::PnLData {
        total_users: 0,
        total_volume: "0".to_string(),
        total_profits: "0".to_string(),
        total_losses: "0".to_string(),
        block_number: blk.number,
        block_timestamp: Some(blk.timestamp().to_owned()),
        ..Default::default()
    };

    let mut user_pnls: HashMap<String, contract::UserPnL> = HashMap::new();
    let mut market_pnls: HashMap<String, contract::MarketPnL> = HashMap::new();
    let mut total_volume = 0u64;
    let mut total_trades = 0u64;

    // Process CTF events for P&L calculations
    for receipt in blk.receipts() {
        for log in &receipt.receipt.logs {
            if log.address == CTF_CONTRACT {
                // Process TransferSingle events for volume tracking
                if let Some(event) = abi::profitandloss_contract::events::TransferSingle::match_and_decode(log) {
                    let from_addr = Hex(&event.from).to_string();
                    let to_addr = Hex(&event.to).to_string();
                    let value: u64 = event.value.to_u64();

                    // Skip zero transfers and mint/burn events
                    if value == 0 || from_addr == "0x0000000000000000000000000000000000000000" || to_addr == "0x0000000000000000000000000000000000000000" {
                        continue;
                    }

                    total_volume += value;
                    total_trades += 1;

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
                                        // Enhanced P&L fields
                                        net_usdc: "0".to_string(),
                                        share_value: "0".to_string(),
                                        trading_pnl: "0".to_string(),
                                        liq_pnl: "0".to_string(),
                                        total_pnl: "0".to_string(),
                                        holdings: Vec::new(),
                                        // Risk Management fields
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
                                }
                            });

                            // Update user metrics
                            let current_volume: u64 = user_pnl.total_volume.parse().unwrap_or(0);
                            user_pnl.total_volume = (current_volume + value).to_string();
                            
                            let current_trades: u64 = user_pnl.total_trades.parse().unwrap_or(0);
                            user_pnl.total_trades = (current_trades + 1).to_string();
                            
                            user_pnl.last_activity = Some(blk.timestamp().to_owned());
                        }
                    }
                }

                // Process Condition Resolution for market P&L
                if let Some(event) = abi::profitandloss_contract::events::ConditionResolution::match_and_decode(log) {
                    let condition_id = Hex(&event.condition_id).to_string();
                    let market_pnl = market_pnls.entry(condition_id.clone()).or_insert_with(|| {
                        contract::MarketPnL {
                            condition_id: condition_id.clone(),
                            question_id: Hex(&event.question_id).to_string(),
                            total_volume: "0".to_string(),
                            total_trades: "0".to_string(),
                            total_fees: "0".to_string(),
                            winning_outcome: "0".to_string(),
                            resolution_price: "0".to_string(),
                            created_at: Some(blk.timestamp().to_owned()),
                            resolved_at: Some(blk.timestamp().to_owned()),
                        }
                    });

                    market_pnl.resolved_at = Some(blk.timestamp().to_owned());
                    market_pnl.winning_outcome = "0".to_string(); // Would need to determine from payouts
                }
            }
        }
    }

    // Convert HashMaps to Vecs
    pnl_data.user_pnls = user_pnls.into_values().collect();
    pnl_data.market_pnls = market_pnls.into_values().collect();
    pnl_data.total_users = pnl_data.user_pnls.len() as u64;
    pnl_data.total_volume = total_volume.to_string();

    // Add global P&L summary
    pnl_data.global_pnls.push(contract::GlobalPnL {
        total_volume: total_volume.to_string(),
        total_trades: total_trades.to_string(),
        total_fees: "0".to_string(),
        active_users: pnl_data.total_users.to_string(),
        active_markets: pnl_data.market_pnls.len().to_string(),
        resolved_markets: pnl_data.market_pnls.len().to_string(),
        timestamp: Some(blk.timestamp().to_owned()),
    });

    Ok(pnl_data)
}

// Price Tracking Handler - Extract prices from OrderFilled events
#[substreams::handlers::map]
fn map_price_data(blk: eth::Block) -> Result<contract::PriceData, substreams::errors::Error> {
    let price_data = contract::PriceData::default();
    
    for receipt in blk.receipts() {
        for log in &receipt.receipt.logs {
            if log.address == CTF_EXCHANGE_CONTRACT {
                // Note: This would need CTF Exchange ABI to properly decode OrderFilled events
                // For now, we'll create a placeholder that tracks basic price data
                // In a real implementation, you'd decode OrderFilled events and calculate prices
            }
        }
    }
    
    Ok(price_data)
}

// USDC Position Tracking Handler
#[substreams::handlers::map]
fn map_usdc_positions(blk: eth::Block) -> Result<contract::UsdcPosition, substreams::errors::Error> {
    let usdc_positions: HashMap<String, contract::UsdcPosition> = HashMap::new();
    
    for receipt in blk.receipts() {
        for log in &receipt.receipt.logs {
            if log.address == USDC_CONTRACT {
                // Note: This would need USDC ABI to properly decode Transfer events
                // For now, we'll create a placeholder that tracks USDC positions
                // In a real implementation, you'd decode Transfer events and track net USDC per user
            }
        }
    }
    
    // Return the first position or default
    Ok(usdc_positions.into_values().next().unwrap_or_default())
}

// Market Metadata Tracking Handler
#[substreams::handlers::map]
fn map_market_metadata(blk: eth::Block) -> Result<contract::MarketMetadata, substreams::errors::Error> {
    let mut metadata = contract::MarketMetadata::default();
    
    for receipt in blk.receipts() {
        for log in &receipt.receipt.logs {
            if log.address == CTF_CONTRACT {
                // Process Condition Preparation for market metadata
                if let Some(event) = abi::profitandloss_contract::events::ConditionPreparation::match_and_decode(log) {
                    metadata.condition_id = Hex(&event.condition_id).to_string();
                    metadata.question_id = Hex(&event.question_id).to_string();
                    metadata.question = "Unknown Question".to_string(); // Would need external metadata
                    metadata.created_at = Some(blk.timestamp().to_owned());
                    metadata.block_number = blk.number;
                    break; // Return first market found
                }
            }
        }
    }
    
    Ok(metadata)
}

// Enhanced P&L Handler with Dune Query Compatibility
#[substreams::handlers::map]
fn map_enhanced_pnl(blk: eth::Block) -> Result<contract::PnLData, substreams::errors::Error> {
    let mut pnl_data = contract::PnLData {
        total_users: 0,
        total_volume: "0".to_string(),
        total_profits: "0".to_string(),
        total_losses: "0".to_string(),
        block_number: blk.number,
        block_timestamp: Some(blk.timestamp().to_owned()),
        ..Default::default()
    };

    let mut user_pnls: HashMap<String, contract::UserPnL> = HashMap::new();
    let mut market_pnls: HashMap<String, contract::MarketPnL> = HashMap::new();
    let mut total_volume = 0u64;
    let mut total_trades = 0u64;
    let _total_usdc_volume = 0u64;

    // Process CTF events for enhanced P&L calculations
    for receipt in blk.receipts() {
        for log in &receipt.receipt.logs {
            if log.address == CTF_CONTRACT {
                // Process TransferSingle events for volume tracking
                if let Some(event) = abi::profitandloss_contract::events::TransferSingle::match_and_decode(log) {
                    let from_addr = Hex(&event.from).to_string();
                    let to_addr = Hex(&event.to).to_string();
                    let value: u64 = event.value.to_u64();

                    // Skip zero transfers and mint/burn events
                    if value == 0 || from_addr == "0x0000000000000000000000000000000000000000" || to_addr == "0x0000000000000000000000000000000000000000" {
                        continue;
                    }

                    total_volume += value;
                    total_trades += 1;

                    // Track enhanced user P&L with Dune query compatibility
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
                                        // Enhanced P&L fields matching Dune query
                                        net_usdc: "0".to_string(),
                                        share_value: "0".to_string(),
                                        trading_pnl: "0".to_string(),
                                        liq_pnl: "0".to_string(),
                                        total_pnl: "0".to_string(),
                                        holdings: Vec::new(),
                                        // Risk Management fields
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
                                }
                            });

                            // Update user metrics
                            let current_volume: u64 = user_pnl.total_volume.parse().unwrap_or(0);
                            user_pnl.total_volume = (current_volume + value).to_string();
                            
                            let current_trades: u64 = user_pnl.total_trades.parse().unwrap_or(0);
                            user_pnl.total_trades = (current_trades + 1).to_string();
                            
                            user_pnl.last_activity = Some(blk.timestamp().to_owned());

                            // Calculate enhanced P&L metrics (simplified for now)
                            // In a real implementation, you'd calculate:
                            // - net_usdc: Net USDC in/out per user
                            // - share_value: Current value of token holdings (price * amount)
                            // - trading_pnl: net_usdc + share_value
                            // - total_pnl: trading_pnl + liq_pnl
                            
                            // Placeholder calculations
                            user_pnl.net_usdc = "0".to_string(); // Would need USDC tracking
                            user_pnl.share_value = "0".to_string(); // Would need price * amount
                            user_pnl.trading_pnl = "0".to_string(); // Would need net_usdc + share_value
                            user_pnl.liq_pnl = "0".to_string(); // Would need airdrop/rewards tracking
                            user_pnl.total_pnl = "0".to_string(); // Would need trading_pnl + liq_pnl
                        }
                    }
                }

                // Process Condition Resolution for market P&L
                if let Some(event) = abi::profitandloss_contract::events::ConditionResolution::match_and_decode(log) {
                    let condition_id = Hex(&event.condition_id).to_string();
                    let market_pnl = market_pnls.entry(condition_id.clone()).or_insert_with(|| {
                        contract::MarketPnL {
                            condition_id: condition_id.clone(),
                            question_id: Hex(&event.question_id).to_string(),
                            total_volume: "0".to_string(),
                            total_trades: "0".to_string(),
                            total_fees: "0".to_string(),
                            winning_outcome: "0".to_string(),
                            resolution_price: "0".to_string(),
                            created_at: Some(blk.timestamp().to_owned()),
                            resolved_at: Some(blk.timestamp().to_owned()),
                        }
                    });

                    market_pnl.resolved_at = Some(blk.timestamp().to_owned());
                    market_pnl.winning_outcome = "0".to_string(); // Would need to determine from payouts
                }
            }
        }
    }

    // Convert HashMaps to Vecs
    pnl_data.user_pnls = user_pnls.into_values().collect();
    pnl_data.market_pnls = market_pnls.into_values().collect();
    pnl_data.total_users = pnl_data.user_pnls.len() as u64;
    pnl_data.total_volume = total_volume.to_string();

    // Add global P&L summary
    pnl_data.global_pnls.push(contract::GlobalPnL {
        total_volume: total_volume.to_string(),
        total_trades: total_trades.to_string(),
        total_fees: "0".to_string(),
        active_users: pnl_data.total_users.to_string(),
        active_markets: pnl_data.market_pnls.len().to_string(),
        resolved_markets: pnl_data.market_pnls.len().to_string(),
        timestamp: Some(blk.timestamp().to_owned()),
    });

    Ok(pnl_data)
}
// Negative Risk Exchange Events Handler
#[substreams::handlers::map]
fn map_neg_risk_events(blk: eth::Block) -> Result<contract::NegRiskEvents, substreams::errors::Error> {
    let mut events = contract::NegRiskEvents::default();

    for receipt in blk.receipts() {
        for log in &receipt.receipt.logs {
            if log.address == NEG_RISK_EXCHANGE {
                // Note: These would need the NegRisk Exchange ABI to be properly implemented
                // For now, we'll create placeholder handlers for risk events
                // In a real implementation, you'd decode specific risk events
            }
        }
    }

    Ok(events)
}

// NegRisk Adapter Events Handler
#[substreams::handlers::map]
fn map_neg_risk_adapter_events(blk: eth::Block) -> Result<contract::NegRiskEvents, substreams::errors::Error> {
    let mut events = contract::NegRiskEvents::default();

    for receipt in blk.receipts() {
        for log in &receipt.receipt.logs {
            if log.address == NEG_RISK_ADAPTER {
                // Note: These would need the NegRisk Adapter ABI to be properly implemented
                // For now, we'll create placeholder handlers for risk events
                // In a real implementation, you'd decode specific risk events
            }
        }
    }

    Ok(events)
}

// NegRisk Wrapped Collateral Events Handler
#[substreams::handlers::map]
fn map_neg_risk_collateral_events(blk: eth::Block) -> Result<contract::NegRiskEvents, substreams::errors::Error> {
    let mut events = contract::NegRiskEvents::default();

    for receipt in blk.receipts() {
        for log in &receipt.receipt.logs {
            if log.address == NEG_RISK_WRAPPED_COLLATERAL {
                // Note: These would need the NegRisk Wrapped Collateral ABI to be properly implemented
                // For now, we'll create placeholder handlers for risk events
                // In a real implementation, you'd decode specific risk events
            }
        }
    }

    Ok(events)
}

// Comprehensive Risk Management Handler
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
    let mut market_risks: HashMap<String, contract::MarketRiskProfile> = HashMap::new();
    let mut total_exposure = 0u64;
    let mut high_risk_users = 0u64;

    // Process all contracts for risk assessment
    for receipt in blk.receipts() {
        for log in &receipt.receipt.logs {
            // Process CTF events for position risk
            if log.address == CTF_CONTRACT {
                if let Some(event) = abi::profitandloss_contract::events::TransferSingle::match_and_decode(log) {
                    let from_addr = Hex(&event.from).to_string();
                    let to_addr = Hex(&event.to).to_string();
                    let value: u64 = event.value.to_u64();

                    // Skip zero transfers and mint/burn events
                    if value == 0 || from_addr == "0x0000000000000000000000000000000000000000" || to_addr == "0x0000000000000000000000000000000000000000" {
                        continue;
                    }

                    total_exposure += value;

                    // Calculate risk metrics for each user
                    for addr in [&from_addr, &to_addr] {
                        if addr != "0x0000000000000000000000000000000000000000" {
                            let user_risk = user_risks.entry(addr.clone()).or_insert_with(|| {
                                contract::UserRiskProfile {
                                    user_address: addr.clone(),
                                    risk_metrics: Some(contract::RiskMetrics {
                                        total_exposure: "0".to_string(),
                                        max_position_size: "0".to_string(),
                                        portfolio_concentration: "0".to_string(),
                                        leverage_ratio: "1.0".to_string(),
                                        margin_ratio: "1.0".to_string(),
                                        liquidation_risk: "0".to_string(),
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

                            // Update exposure
                            let current_exposure: u64 = user_risk.current_exposure.parse().unwrap_or(0);
                            user_risk.current_exposure = (current_exposure + value).to_string();

                            // Calculate risk score (simplified)
                            let exposure_value: f64 = user_risk.current_exposure.parse().unwrap_or(0) as f64;
                            let risk_score = if exposure_value > 1000000.0 { 80.0 } // High risk for > 1M
                                else if exposure_value > 100000.0 { 60.0 } // Medium risk for > 100K
                                else { 20.0 }; // Low risk
                            
                            user_risk.current_risk_score = risk_score.to_string();

                            // Check for high risk users
                            if risk_score > 70.0 {
                                high_risk_users += 1;
                                
                                // Add risk alert
                                user_risk.active_alerts.push(contract::RiskAlert {
                                    alert_type: "high_exposure".to_string(),
                                    severity: if risk_score > 80.0 { "critical".to_string() } else { "high".to_string() },
                                    message: format!("High exposure detected: {} tokens", user_risk.current_exposure),
                                    value: user_risk.current_exposure.clone(),
                                    threshold: "1000000".to_string(),
                                    triggered_at: Some(blk.timestamp().to_owned()),
                                    acknowledged: false,
                                });
                            }

                            // Update risk metrics
                            if let Some(ref mut metrics) = user_risk.risk_metrics {
                                metrics.total_exposure = user_risk.current_exposure.clone();
                                
                                // Calculate portfolio concentration (simplified)
                                let concentration = if exposure_value > 0.0 {
                                    (exposure_value / (exposure_value + 100000.0) * 100.0).to_string()
                                } else {
                                    "0".to_string()
                                };
                                metrics.portfolio_concentration = concentration;
                                
                                // Calculate liquidation risk (simplified)
                                let liquidation_risk = if risk_score > 80.0 { "high".to_string() }
                                    else if risk_score > 60.0 { "medium".to_string() }
                                    else { "low".to_string() };
                                metrics.liquidation_risk = liquidation_risk;
                            }

                            user_risk.last_assessment = Some(blk.timestamp().to_owned());
                        }
                    }
                }
            }

            // Process NegRisk contracts for additional risk data
            if log.address == NEG_RISK_EXCHANGE || log.address == NEG_RISK_ADAPTER || log.address == NEG_RISK_WRAPPED_COLLATERAL {
                // Note: These would need specific ABIs to decode risk events
                // For now, we'll track basic risk exposure
            }
        }
    }

    // Convert HashMaps to Vecs
    risk_data.user_risks = user_risks.into_values().collect();
    risk_data.market_risks = market_risks.into_values().collect();
    risk_data.total_risk_exposure = total_exposure.to_string();

    // Add global risk metrics
    risk_data.global_risks.push(contract::GlobalRiskMetrics {
        total_system_exposure: total_exposure.to_string(),
        average_risk_score: if risk_data.user_risks.len() > 0 {
            (risk_data.user_risks.iter()
                .map(|u| u.current_risk_score.parse::<f64>().unwrap_or(0.0))
                .sum::<f64>() / risk_data.user_risks.len() as f64).to_string()
        } else {
            "0".to_string()
        },
        high_risk_users_count: high_risk_users.to_string(),
        total_liquidations: "0".to_string(),
        risk_fees_collected: "0".to_string(),
        system_stability_score: if high_risk_users > 0 { "70".to_string() } else { "90".to_string() },
        timestamp: Some(blk.timestamp().to_owned()),
    });

    // Calculate system risk score
    let system_risk = if high_risk_users > 0 {
        (high_risk_users as f64 / risk_data.user_risks.len().max(1) as f64 * 100.0) as u64
    } else {
        0
    };
    risk_data.system_risk_score = system_risk.to_string();

    Ok(risk_data)
}
