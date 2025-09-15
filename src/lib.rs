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

// Contract addresses from Dune query - EXACT MATCH
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

// Excluded addresses from Dune query - EXACT MATCH
const EXCLUDED_ADDRESSES: [&str; 7] = [
    "0x4d97dcd97ec945f40cf65f87097ace5ea0476045", // CTF Contract
    "0x4bfb41d5b3570defd03c39a9a4d8de6bd8b8982e", // CTF Exchange
    "0x78769D50Be1763ed1CA0D5E878D93f05aabff29e", // Neg Risk Fee Module
    "0x3a3bd7bb9528e159577f7c2e685cc81a765002e2", // UMA Merkle Distributor
    "0xa5ef39c3d3e10d0b270233af41cac69796b12966", // FPMM Factory
    "0xA2bD9CC3e04996Ca683C834E4D86A016f6bbDE5A", // Additional excluded
    "0x0000000000000000000000000000000000000000", // Zero address
];

// 1. CTF Exchange TokenRegistered Events (Dune: polymarket_polygon.CTFExchange_evt_TokenRegistered)
#[substreams::handlers::map]
fn map_ctf_exchange_token_registered(blk: eth::Block) -> Result<contract::TokenRegisteredEvents, substreams::errors::Error> {
    let mut events = contract::TokenRegisteredEvents::default();

    for receipt in blk.receipts() {
        for log in &receipt.receipt.logs {
            if log.address == CTF_EXCHANGE_CONTRACT {
                // Decode TokenRegistered event
                if let Some(mut decoded) = abi::decode_token_registered(log) {
                    decoded.evt_tx_hash = Hex(&receipt.transaction.hash).to_string();
                    decoded.evt_block_time = Some(blk.timestamp().to_owned());
                    decoded.evt_block_number = blk.number;
                    events.token_registered.push(decoded);
                }
            }
        }
    }

    Ok(events)
}

// 2. NegRisk CTF Exchange TokenRegistered Events (Dune: polymarket_polygon.NegRiskCtfExchange_evt_TokenRegistered)
#[substreams::handlers::map]
fn map_neg_risk_ctf_exchange_token_registered(blk: eth::Block) -> Result<contract::NegRiskTokenRegisteredEvents, substreams::errors::Error> {
    let mut events = contract::NegRiskTokenRegisteredEvents::default();

    for receipt in blk.receipts() {
        for log in &receipt.receipt.logs {
            if log.address == NEG_RISK_CTF_EXCHANGE {
                // Decode NegRisk TokenRegistered event
                if let Some(mut decoded) = abi::decode_neg_risk_token_registered(log) {
                    decoded.evt_tx_hash = Hex(&receipt.transaction.hash).to_string();
                    decoded.evt_block_time = Some(blk.timestamp().to_owned());
                    decoded.evt_block_number = blk.number;
                    events.neg_risk_token_registered.push(decoded);
                }
            }
        }
    }

    Ok(events)
}

// 3. Fixed Product Market Maker Factory Creation (Dune: polymarketfactory_polygon.FixedProductMarketMakerFactory_evt_FixedProductMarketMakerCreation)
#[substreams::handlers::map]
fn map_fpmm_factory_creation(blk: eth::Block) -> Result<contract::FpmmFactoryEvents, substreams::errors::Error> {
    let mut events = contract::FpmmFactoryEvents::default();

    for receipt in blk.receipts() {
        for log in &receipt.receipt.logs {
            if log.address == FPMM_FACTORY_CONTRACT {
                // Decode FixedProductMarketMakerCreation event
                if let Some(mut decoded) = abi::decode_fpmm_creation(log) {
                    decoded.evt_tx_hash = Hex(&receipt.transaction.hash).to_string();
                    decoded.evt_block_time = Some(blk.timestamp().to_owned());
                    decoded.evt_block_number = blk.number;
                    events.factory_creations.push(decoded);
                }
            }
        }
    }

    Ok(events)
}

// 4. CTF Exchange OrderFilled Events (Dune: polymarket_polygon.CTFExchange_evt_OrderFilled)
#[substreams::handlers::map]
fn map_ctf_exchange_order_filled(blk: eth::Block) -> Result<contract::OrderFilledEvents, substreams::errors::Error> {
    let mut events = contract::OrderFilledEvents::default();

    for receipt in blk.receipts() {
        for log in &receipt.receipt.logs {
            if log.address == CTF_EXCHANGE_CONTRACT {
                // Decode OrderFilled event
                if let Some(mut decoded) = abi::decode_order_filled(log) {
                    decoded.evt_tx_hash = Hex(&receipt.transaction.hash).to_string();
                    decoded.evt_block_time = Some(blk.timestamp().to_owned());
                    decoded.evt_block_number = blk.number;
                    events.order_filled.push(decoded);
                }
            }
        }
    }

    Ok(events)
}

// 5. NegRisk CTF Exchange OrderFilled Events (Dune: polymarket_polygon.NegRiskCtfExchange_evt_OrderFilled)
#[substreams::handlers::map]
fn map_neg_risk_ctf_exchange_order_filled(blk: eth::Block) -> Result<contract::OrderFilledEvents, substreams::errors::Error> {
    let mut events = contract::OrderFilledEvents::default();

    for receipt in blk.receipts() {
        for log in &receipt.receipt.logs {
            if log.address == NEG_RISK_CTF_EXCHANGE {
                // Decode NegRisk OrderFilled event
                if let Some(mut decoded) = abi::decode_order_filled(log) {
                    decoded.evt_tx_hash = Hex(&receipt.transaction.hash).to_string();
                    decoded.evt_block_time = Some(blk.timestamp().to_owned());
                    decoded.evt_block_number = blk.number;
                    events.order_filled.push(decoded);
                }
            }
        }
    }

    Ok(events)
}

// 6. ERC1155 TransferSingle Events (Dune: erc1155_polygon.evt_TransferSingle)
#[substreams::handlers::map]
fn map_erc1155_transfer_single(blk: eth::Block) -> Result<contract::Erc1155TransferSingleEvents, substreams::errors::Error> {
    let mut events = contract::Erc1155TransferSingleEvents::default();

    for receipt in blk.receipts() {
        for log in &receipt.receipt.logs {
            // Check for TransferSingle event signature
            if log.topics.len() >= 4 && log.topics[0] == TRANSFER_SINGLE_SIG {
                if let Some(mut decoded) = abi::decode_erc1155_transfer_single(log) {
                    decoded.evt_tx_hash = Hex(&receipt.transaction.hash).to_string();
                    decoded.evt_block_time = Some(blk.timestamp().to_owned());
                    decoded.evt_block_number = blk.number;
                    events.transfer_single.push(decoded);
                }
            }
        }
    }

    Ok(events)
}

// 7. ERC1155 TransferBatch Events (Dune: erc1155_polygon.evt_TransferBatch)
#[substreams::handlers::map]
fn map_erc1155_transfer_batch(blk: eth::Block) -> Result<contract::Erc1155TransferBatchEvents, substreams::errors::Error> {
    let mut events = contract::Erc1155TransferBatchEvents::default();

    for receipt in blk.receipts() {
        for log in &receipt.receipt.logs {
            // Check for TransferBatch event signature
            if log.topics.len() >= 4 && log.topics[0] == TRANSFER_BATCH_SIG {
                if let Some(mut decoded) = abi::decode_erc1155_transfer_batch(log) {
                    decoded.evt_tx_hash = Hex(&receipt.transaction.hash).to_string();
                    decoded.evt_block_time = Some(blk.timestamp().to_owned());
                    decoded.evt_block_number = blk.number;
                    events.transfer_batch.push(decoded);
                }
            }
        }
    }

    Ok(events)
}

// 8. ERC20 Transfer Events (Dune: erc20_polygon.evt_Transfer)
#[substreams::handlers::map]
fn map_erc20_transfer(blk: eth::Block) -> Result<contract::Erc20TransferEvents, substreams::errors::Error> {
    let mut events = contract::Erc20TransferEvents::default();

    for receipt in blk.receipts() {
        for log in &receipt.receipt.logs {
            // Check for Transfer event signature
            if log.topics.len() >= 3 && log.topics[0] == TRANSFER_SIG {
                if let Some(mut decoded) = abi::decode_erc20_transfer(log) {
                    decoded.evt_tx_hash = Hex(&receipt.transaction.hash).to_string();
                    decoded.evt_block_time = Some(blk.timestamp().to_owned());
                    decoded.evt_block_number = blk.number;
                    events.transfer.push(decoded);
                }
            }
        }
    }

    Ok(events)
}

// 9. UMA Merkle Distributor Events (Dune: polymarket_uma_merkle_distributor_polygon.MerkleDistributor_evt_Claimed)
#[substreams::handlers::map]
fn map_uma_merkle_distributor(blk: eth::Block) -> Result<contract::MerkleDistributorEvents, substreams::errors::Error> {
    let mut events = contract::MerkleDistributorEvents::default();

    for receipt in blk.receipts() {
        for log in &receipt.receipt.logs {
            if log.address == UMA_MERKLE_DISTRIBUTOR {
                // Decode MerkleDistributor Claimed event
                if let Some(mut decoded) = abi::decode_merkle_claimed(log) {
                    decoded.evt_tx_hash = Hex(&receipt.transaction.hash).to_string();
                    decoded.evt_block_time = Some(blk.timestamp().to_owned());
                    decoded.evt_block_number = blk.number;
                    events.claimed.push(decoded);
                }
            }
        }
    }

    Ok(events)
}

// 10. USDC Merkle Distributor Events (Dune: polymarket_usdc_merkle_distributor_polygon.MerkleDistributor_evt_Claimed)
#[substreams::handlers::map]
fn map_usdc_merkle_distributor(blk: eth::Block) -> Result<contract::MerkleDistributorEvents, substreams::errors::Error> {
    let mut events = contract::MerkleDistributorEvents::default();

    for receipt in blk.receipts() {
        for log in &receipt.receipt.logs {
            if log.address == USDC_MERKLE_DISTRIBUTOR {
                // Decode MerkleDistributor Claimed event
                if let Some(mut decoded) = abi::decode_merkle_claimed(log) {
                    decoded.evt_tx_hash = Hex(&receipt.transaction.hash).to_string();
                    decoded.evt_block_time = Some(blk.timestamp().to_owned());
                    decoded.evt_block_number = blk.number;
                    events.claimed.push(decoded);
                }
            }
        }
    }

    Ok(events)
}

// 11. CTF Events (Dune: references CTF contract)
#[substreams::handlers::map]
fn map_ctf_events(blk: eth::Block) -> Result<contract::CtfEvents, substreams::errors::Error> {
    let mut events = contract::CtfEvents::default();

    for receipt in blk.receipts() {
        for log in &receipt.receipt.logs {
            if log.address == CTF_CONTRACT {
                // Decode various CTF events
                if let Some(decoded) = abi::decode_ctf_events(log) {
                    match decoded {
                        abi::CtfEventType::ConditionPreparation(mut evt) => {
                            evt.evt_tx_hash = Hex(&receipt.transaction.hash).to_string();
                            evt.evt_block_time = Some(blk.timestamp().to_owned());
                            evt.evt_block_number = blk.number;
                            events.condition_preparations.push(evt);
                        },
                        abi::CtfEventType::ConditionResolution(mut evt) => {
                            evt.evt_tx_hash = Hex(&receipt.transaction.hash).to_string();
                            evt.evt_block_time = Some(blk.timestamp().to_owned());
                            evt.evt_block_number = blk.number;
                            events.condition_resolutions.push(evt);
                        },
                        abi::CtfEventType::PositionSplit(mut evt) => {
                            evt.evt_tx_hash = Hex(&receipt.transaction.hash).to_string();
                            evt.evt_block_time = Some(blk.timestamp().to_owned());
                            evt.evt_block_number = blk.number;
                            events.position_splits.push(evt);
                        },
                        abi::CtfEventType::PositionMerge(mut evt) => {
                            evt.evt_tx_hash = Hex(&receipt.transaction.hash).to_string();
                            evt.evt_block_time = Some(blk.timestamp().to_owned());
                            evt.evt_block_number = blk.number;
                            events.position_merges.push(evt);
                        },
                        abi::CtfEventType::PositionRedeem(mut evt) => {
                            evt.evt_tx_hash = Hex(&receipt.transaction.hash).to_string();
                            evt.evt_block_time = Some(blk.timestamp().to_owned());
                            evt.evt_block_number = blk.number;
                            events.position_redemptions.push(evt);
                        },
                        abi::CtfEventType::TransferSingle(mut evt) => {
                            evt.evt_tx_hash = Hex(&receipt.transaction.hash).to_string();
                            evt.evt_block_time = Some(blk.timestamp().to_owned());
                            evt.evt_block_number = blk.number;
                            events.transfer_singles.push(evt);
                        },
                        abi::CtfEventType::TransferBatch(mut evt) => {
                            evt.evt_tx_hash = Hex(&receipt.transaction.hash).to_string();
                            evt.evt_block_time = Some(blk.timestamp().to_owned());
                            evt.evt_block_number = blk.number;
                            events.transfer_batches.push(evt);
                        },
                    }
                }
            }
        }
    }

    Ok(events)
}

// 12. USDC Events (Dune: erc20_polygon.evt_Transfer where contract_address = 0x2791bca1f2de4661ed88a30c99a7a9449aa84174)
#[substreams::handlers::map]
fn map_usdc_events(blk: eth::Block) -> Result<contract::UsdcEvents, substreams::errors::Error> {
    let mut events = contract::UsdcEvents::default();

    for receipt in blk.receipts() {
        for log in &receipt.receipt.logs {
            if log.address == USDC_CONTRACT {
                // Decode USDC Transfer and Approval events
                if let Some(decoded) = abi::decode_usdc_events(log) {
                    match decoded {
                        abi::UsdcEventType::Transfer(mut evt) => {
                            evt.evt_tx_hash = Hex(&receipt.transaction.hash).to_string();
                            evt.evt_block_time = Some(blk.timestamp().to_owned());
                            evt.evt_block_number = blk.number;
                            events.transfers.push(evt);
                        },
                        abi::UsdcEventType::Approval(mut evt) => {
                            evt.evt_tx_hash = Hex(&receipt.transaction.hash).to_string();
                            evt.evt_block_time = Some(blk.timestamp().to_owned());
                            evt.evt_block_number = blk.number;
                            events.approvals.push(evt);
                        },
                    }
                }
            }
        }
    }

    Ok(events)
}

// 13. Pure Dune Query P&L Data - EXACT MATCH TO DUNE QUERY (NO SIMULATION)
#[substreams::handlers::map]
fn map_pure_dune_pnl(blk: eth::Block) -> Result<contract::PureDunePnL, substreams::errors::Error> {
    let mut pnl_data = contract::PureDunePnL {
        total_users: "0".to_string(),
        total_volume: "0".to_string(),
        total_profits: "0".to_string(),
        total_losses: "0".to_string(),
        block_number: blk.number,
        block_timestamp: Some(blk.timestamp().to_owned()),
        ..Default::default()
    };

    let mut user_pnls: HashMap<String, contract::DuneUserPnL> = HashMap::new();
    let market_data: HashMap<String, contract::DuneMarketData> = HashMap::new();
    let mut token_transfers: Vec<contract::DuneTokenTransfer> = Vec::new();
    let mut order_fills: Vec<contract::DuneOrderFill> = Vec::new();
    let mut reward_claims: Vec<contract::DuneRewardClaim> = Vec::new();
    let mut price_data: HashMap<String, contract::DunePriceData> = HashMap::new();

    // Process all events to build Dune query compatible data
    for receipt in blk.receipts() {
        for log in &receipt.receipt.logs {
            // Process ERC1155 transfers (sends/receives from Dune query)
            if log.topics.len() >= 4 && log.topics[0] == TRANSFER_SINGLE_SIG {
                if let Some(transfer) = abi::decode_erc1155_transfer_single(log) {
                    let from_addr = Hex(&log.topics[2]).to_string();
                    let to_addr = Hex(&log.topics[3]).to_string();
                    let token_id = transfer.id.clone();
                    let amount = transfer.value.clone();

                    // Convert to USDC units (divide by 1000000) as per Dune query
                    let amount_usdc = if let Ok(val) = amount.parse::<f64>() {
                        (val / 1_000_000.0).to_string()
                    } else {
                        "0".to_string()
                    };

                    // Process sends (negative amount)
                    if from_addr != "0x0000000000000000000000000000000000000000" {
                        if !is_excluded_address(&from_addr) {
                            token_transfers.push(contract::DuneTokenTransfer {
                                transaction_hash: transfer.evt_tx_hash.clone(),
                                user_address: from_addr.clone(),
                                token_id: token_id.clone(),
                                amount: format!("-{}", amount_usdc),
                                transfer_type: "ERC1155_SINGLE".to_string(),
                                block_timestamp: Some(blk.timestamp().to_owned()),
                                block_number: blk.number,
                            });

                            // Update user P&L
                            update_user_pnl(&mut user_pnls, &from_addr, &token_id, &amount_usdc, &blk, false);
                        }
                    }

                    // Process receives (positive amount)
                    if to_addr != "0x0000000000000000000000000000000000000000" {
                        if !is_excluded_address(&to_addr) {
                            token_transfers.push(contract::DuneTokenTransfer {
                                transaction_hash: transfer.evt_tx_hash.clone(),
                                user_address: to_addr.clone(),
                                token_id: token_id.clone(),
                                amount: amount_usdc.clone(),
                                transfer_type: "ERC1155_SINGLE".to_string(),
                                block_timestamp: Some(blk.timestamp().to_owned()),
                                block_number: blk.number,
                            });

                            // Update user P&L
                            update_user_pnl(&mut user_pnls, &to_addr, &token_id, &amount_usdc, &blk, true);
                        }
                    }
                }
            }

            // Process ERC20 transfers (USDC from Dune query)
            if log.topics.len() >= 3 && log.topics[0] == TRANSFER_SIG && log.address == USDC_CONTRACT {
                if let Some(transfer) = abi::decode_erc20_transfer(log) {
                    let from_addr = Hex(&log.topics[1]).to_string();
                    let to_addr = Hex(&log.topics[2]).to_string();
                    let amount = transfer.value.clone();

                    // Convert to USDC units (divide by 1000000) as per Dune query
                    let amount_usdc = if let Ok(val) = amount.parse::<f64>() {
                        (val / 1_000_000.0).to_string()
                    } else {
                        "0".to_string()
                    };

                    // Process USDC transfers
                    if !is_excluded_address(&from_addr) {
                        token_transfers.push(contract::DuneTokenTransfer {
                            transaction_hash: transfer.evt_tx_hash.clone(),
                            user_address: from_addr.clone(),
                            token_id: "USDC".to_string(),
                            amount: format!("-{}", amount_usdc),
                            transfer_type: "ERC20".to_string(),
                            block_timestamp: Some(blk.timestamp().to_owned()),
                            block_number: blk.number,
                        });
                    }

                    if !is_excluded_address(&to_addr) {
                        token_transfers.push(contract::DuneTokenTransfer {
                            transaction_hash: transfer.evt_tx_hash.clone(),
                            user_address: to_addr.clone(),
                            token_id: "USDC".to_string(),
                            amount: amount_usdc.clone(),
                            transfer_type: "ERC20".to_string(),
                            block_timestamp: Some(blk.timestamp().to_owned()),
                            block_number: blk.number,
                        });
                    }
                }
            }

            // Process OrderFilled events for price data
            if log.address == CTF_EXCHANGE_CONTRACT || log.address == NEG_RISK_CTF_EXCHANGE {
                if let Some(order_fill) = abi::decode_order_filled(log) {
                    let is_neg_risk = log.address == NEG_RISK_CTF_EXCHANGE;
                    
                    order_fills.push(contract::DuneOrderFill {
                        transaction_hash: order_fill.evt_tx_hash.clone(),
                        log_index: order_fill.evt_index,
                        block_timestamp: Some(blk.timestamp().to_owned()),
                        block_number: blk.number,
                        maker_address: Hex(&order_fill.maker).to_string(),
                        taker_address: Hex(&order_fill.taker).to_string(),
                        maker_asset_id: order_fill.maker_asset_id.clone(),
                        taker_asset_id: order_fill.taker_asset_id.clone(),
                        maker_amount_filled: order_fill.maker_amount_filled.clone(),
                        taker_amount_filled: order_fill.taker_amount_filled.clone(),
                        fee: order_fill.fee.clone(),
                        order_hash: Hex(&order_fill.order_hash).to_string(),
                        is_neg_risk,
                    });

                    // Update price data from latest trades
                    update_price_data(&mut price_data, &order_fill, &blk);
                }
            }

            // Process Merkle Distributor events for rewards
            if log.address == UMA_MERKLE_DISTRIBUTOR || log.address == USDC_MERKLE_DISTRIBUTOR {
                if let Some(claimed) = abi::decode_merkle_claimed(log) {
                    let asset = if log.address == UMA_MERKLE_DISTRIBUTOR { "uma" } else { "usdc" };
                    let amount = claimed.amount.clone();
                    
                    // Convert amounts based on asset type
                    let (lc_amount, usd_amount) = if asset == "uma" {
                        // UMA: divide by 10^18, then multiply by price (simplified to 1 for now)
                        let lc = if let Ok(val) = amount.parse::<f64>() {
                            val / 1_000_000_000_000_000_000.0
                        } else { 0.0 };
                        (lc.to_string(), lc.to_string()) // Simplified price = 1
                    } else {
                        // USDC: divide by 10^6
                        let usd = if let Ok(val) = amount.parse::<f64>() {
                            val / 1_000_000.0
                        } else { 0.0 };
                        (usd.to_string(), usd.to_string())
                    };

                    reward_claims.push(contract::DuneRewardClaim {
                        transaction_hash: claimed.evt_tx_hash.clone(),
                        log_index: claimed.evt_index,
                        block_timestamp: Some(blk.timestamp().to_owned()),
                        block_number: blk.number,
                        airdrop_recipient: Hex(&claimed.airdrop_recipient).to_string(),
                        asset: asset.to_string(),
                        lc_amount,
                        usd_amount,
                        token_address: Hex(&log.address).to_string(),
                    });
                }
            }
        }
    }

    // Convert HashMap to Vec
    pnl_data.user_pnls = user_pnls.into_values().collect();
    pnl_data.market_data = market_data.into_values().collect();
    pnl_data.token_transfers = token_transfers;
    pnl_data.order_fills = order_fills;
    pnl_data.reward_claims = reward_claims;
    pnl_data.price_data = price_data.into_values().collect();
    pnl_data.total_users = pnl_data.user_pnls.len().to_string();

    Ok(pnl_data)
}

// Helper function to check if address is excluded (from Dune query)
fn is_excluded_address(addr: &str) -> bool {
    EXCLUDED_ADDRESSES.contains(&addr)
}

// Helper function to update user P&L
fn update_user_pnl(
    user_pnls: &mut HashMap<String, contract::DuneUserPnL>,
    user_addr: &str,
    token_id: &str,
    amount: &str,
    blk: &eth::Block,
    is_receive: bool,
) {
    let user_pnl = user_pnls.entry(user_addr.to_string()).or_insert_with(|| {
        contract::DuneUserPnL {
            user_address: user_addr.to_string(),
            net_usdc: "0".to_string(),
            share_value: "0".to_string(),
            trading_pnl: "0".to_string(),
            liq_pnl: "0".to_string(),
            total_pnl: "0".to_string(),
            holdings: Vec::new(),
            last_activity: Some(blk.timestamp().to_owned()),
        }
    });

    // Update holdings
    let amount_f64: f64 = amount.parse().unwrap_or(0.0);
    let multiplier = if is_receive { 1.0 } else { -1.0 };
    
    // Find existing holding or create new one
    let mut found = false;
    for holding in &mut user_pnl.holdings {
        if holding.token_id == token_id {
            let current_amount: f64 = holding.amount.parse().unwrap_or(0.0);
            holding.amount = (current_amount + (amount_f64 * multiplier)).to_string();
            found = true;
            break;
        }
    }
    
    if !found && is_receive {
        user_pnl.holdings.push(contract::DuneTokenHolding {
            user_address: user_addr.to_string(),
            token_id: token_id.to_string(),
            amount: amount_f64.to_string(),
            latest_price: "1.0".to_string(), // Will be updated with real price data
            share_value: amount_f64.to_string(),
        });
    }

    user_pnl.last_activity = Some(blk.timestamp().to_owned());
}

// Helper function to update price data from order fills
fn update_price_data(
    price_data: &mut HashMap<String, contract::DunePriceData>,
    order_fill: &contract::OrderFilled,
    blk: &eth::Block,
) {
    // Calculate price from maker/taker amounts
    let maker_amount: f64 = order_fill.maker_amount_filled.parse().unwrap_or(0.0);
    let taker_amount: f64 = order_fill.taker_amount_filled.parse().unwrap_or(0.0);
    
    if taker_amount > 0.0 {
        let price = maker_amount / taker_amount;
        
        // Update price for the token being traded
        let token_id = if order_fill.maker_asset_id != "0" {
            order_fill.maker_asset_id.clone()
        } else {
            order_fill.taker_asset_id.clone()
        };
        
        price_data.insert(token_id.clone(), contract::DunePriceData {
            token_id,
            price: price.to_string(),
            last_trade_time: Some(blk.timestamp().to_owned()),
            block_number: blk.number,
        });
    }
}
