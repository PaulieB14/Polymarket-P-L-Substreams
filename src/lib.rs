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