use crate::pb::contract::v1 as contract;
use substreams::Hex;
use substreams_ethereum::pb::eth::v2::Log;

// Helper function to decode uint256 from bytes
fn u256_from_bytes(bytes: &[u8]) -> u64 {
    if bytes.len() >= 32 {
        // Take the last 8 bytes for u64 (assuming values fit in u64)
        let mut result = 0u64;
        for i in 24..32 {
            result = (result << 8) | bytes[i] as u64;
        }
        result
    } else {
        0
    }
}

// ABI decoding functions for all Dune query events

pub fn decode_token_registered(log: &Log) -> Option<contract::TokenRegistered> {
    // Decode TokenRegistered event from CTF Exchange
    // Event signature: TokenRegistered(bytes32 indexed conditionId, address indexed token0, address indexed token1)
    if log.topics.len() >= 4 {
        Some(contract::TokenRegistered {
            evt_tx_hash: "0x".to_string(), // Will be set by caller
            evt_index: log.block_index,
            evt_block_time: None, // Will be set by caller
            evt_block_number: 0, // Will be set by caller
            condition_id: log.topics[1].to_vec(),
            token0: Hex(&log.topics[2]).to_string(),
            token1: Hex(&log.topics[3]).to_string(),
        })
    } else {
        None
    }
}

pub fn decode_neg_risk_token_registered(log: &Log) -> Option<contract::NegRiskTokenRegistered> {
    // Decode NegRisk TokenRegistered event
    if log.topics.len() >= 4 {
        Some(contract::NegRiskTokenRegistered {
            evt_tx_hash: "0x".to_string(),
            evt_index: log.block_index,
            evt_block_time: None,
            evt_block_number: 0,
            condition_id: log.topics[1].to_vec(),
            token0: Hex(&log.topics[2]).to_string(),
            token1: Hex(&log.topics[3]).to_string(),
            is_neg_risk: true,
            is_augmented: true,
            event_id: "0x".to_string(), // Would need to decode from data
        })
    } else {
        None
    }
}

pub fn decode_fpmm_creation(log: &Log) -> Option<contract::FpmmFactoryCreation> {
    // Decode FixedProductMarketMakerCreation event
    // Event signature: FixedProductMarketMakerCreation(address indexed fixedProductMarketMaker, address indexed creator, bytes32[] conditionIds, address collateralToken, uint256 fee, uint256 endTime, bytes32 questionId)
    if log.topics.len() >= 3 {
        Some(contract::FpmmFactoryCreation {
            evt_tx_hash: "0x".to_string(),
            evt_index: log.block_index,
            evt_block_time: None,
            evt_block_number: 0,
            fixed_product_market_maker: log.topics[1].to_vec(),
            creator: log.topics[2].to_vec(),
            condition_ids: Vec::new(), // Would need to decode from data
            collateral_token: Vec::new(), // Would need to decode from data
            fee: "0".to_string(), // Would need to decode from data
            end_time: "0".to_string(), // Would need to decode from data
            question_id: "0x".to_string(), // Would need to decode from data
        })
    } else {
        None
    }
}

pub fn decode_order_filled(log: &Log) -> Option<contract::OrderFilled> {
    // Decode OrderFilled event
    // Event signature: OrderFilled(address indexed maker, address indexed taker, uint256 makerAssetId, uint256 takerAssetId, uint256 makerAmountFilled, uint256 takerAmountFilled, uint256 fee, bytes32 orderHash)
    if log.topics.len() >= 4 && log.data.len() >= 192 { // 6 * 32 bytes
        // Decode the data: 6 uint256 values (192 bytes total)
        let maker_asset_id = u256_from_bytes(&log.data[0..32]);
        let taker_asset_id = u256_from_bytes(&log.data[32..64]);
        let maker_amount_filled = u256_from_bytes(&log.data[64..96]);
        let taker_amount_filled = u256_from_bytes(&log.data[96..128]);
        let fee = u256_from_bytes(&log.data[128..160]);
        // orderHash is in topics[3]
        
        Some(contract::OrderFilled {
            evt_tx_hash: "0x".to_string(),
            evt_index: log.block_index,
            evt_block_time: None,
            evt_block_number: 0,
            maker: log.topics[1].to_vec(),
            taker: log.topics[2].to_vec(),
            maker_asset_id: maker_asset_id.to_string(),
            taker_asset_id: taker_asset_id.to_string(),
            maker_amount_filled: maker_amount_filled.to_string(),
            taker_amount_filled: taker_amount_filled.to_string(),
            fee: fee.to_string(),
            order_hash: log.topics[3].to_vec(),
        })
    } else {
        None
    }
}

pub fn decode_erc1155_transfer_single(log: &Log) -> Option<contract::Erc1155TransferSingle> {
    // Decode ERC1155 TransferSingle event
    // Event signature: TransferSingle(address indexed operator, address indexed from, address indexed to, uint256 id, uint256 value)
    if log.topics.len() >= 4 && log.data.len() >= 64 {
        // Decode the data: uint256 id (32 bytes) + uint256 value (32 bytes)
        let id_bytes = &log.data[0..32];
        let value_bytes = &log.data[32..64];
        
        let id = u256_from_bytes(id_bytes);
        let value = u256_from_bytes(value_bytes);
        
        Some(contract::Erc1155TransferSingle {
            evt_tx_hash: "0x".to_string(),
            evt_index: log.block_index,
            evt_block_time: None,
            evt_block_number: 0,
            contract_address: log.address.to_vec(),
            operator: log.topics[1].to_vec(),
            from: log.topics[2].to_vec(),
            to: log.topics[3].to_vec(),
            id: id.to_string(),
            value: value.to_string(),
        })
    } else {
        None
    }
}

pub fn decode_erc1155_transfer_batch(log: &Log) -> Option<contract::Erc1155TransferBatch> {
    // Decode ERC1155 TransferBatch event
    // Event signature: TransferBatch(address indexed operator, address indexed from, address indexed to, uint256[] ids, uint256[] values)
    if log.topics.len() >= 4 {
        Some(contract::Erc1155TransferBatch {
            evt_tx_hash: "0x".to_string(),
            evt_index: log.block_index,
            evt_block_time: None,
            evt_block_number: 0,
            contract_address: log.address.to_vec(),
            operator: log.topics[1].to_vec(),
            from: log.topics[2].to_vec(),
            to: log.topics[3].to_vec(),
            ids: Vec::new(), // Would need to decode from data
            values: Vec::new(), // Would need to decode from data
        })
    } else {
        None
    }
}

pub fn decode_erc20_transfer(log: &Log) -> Option<contract::Erc20Transfer> {
    // Decode ERC20 Transfer event
    // Event signature: Transfer(address indexed from, address indexed to, uint256 value)
    if log.topics.len() >= 3 && log.data.len() >= 32 {
        // Decode the data: uint256 value (32 bytes)
        let value_bytes = &log.data[0..32];
        let value = u256_from_bytes(value_bytes);
        
        Some(contract::Erc20Transfer {
            evt_tx_hash: "0x".to_string(),
            evt_index: log.block_index,
            evt_block_time: None,
            evt_block_number: 0,
            contract_address: log.address.to_vec(),
            from: log.topics[1].to_vec(),
            to: log.topics[2].to_vec(),
            value: value.to_string(),
        })
    } else {
        None
    }
}

pub fn decode_merkle_claimed(log: &Log) -> Option<contract::MerkleDistributorClaimed> {
    // Decode MerkleDistributor Claimed event
    // Event signature: Claimed(address indexed airdropRecipient, uint256 amount)
    if log.topics.len() >= 3 {
        Some(contract::MerkleDistributorClaimed {
            evt_tx_hash: "0x".to_string(),
            evt_index: log.block_index,
            evt_block_time: None,
            evt_block_number: 0,
            airdrop_recipient: log.topics[1].to_vec(),
            amount: "0".to_string(), // Would need to decode from data
            token_address: Hex(&log.address).to_string(),
        })
    } else {
        None
    }
}

// CTF Event Types
pub enum CtfEventType {
    ConditionPreparation(contract::CtfConditionPreparation),
    ConditionResolution(contract::CtfConditionResolution),
    PositionSplit(contract::CtfPositionSplit),
    PositionMerge(contract::CtfPositionMerge),
    PositionRedeem(contract::CtfPositionRedeem),
    TransferSingle(contract::CtfTransferSingle),
    TransferBatch(contract::CtfTransferBatch),
}

pub fn decode_ctf_events(log: &Log) -> Option<CtfEventType> {
    // Decode various CTF events based on event signature
    // This is a simplified version - in production you'd decode based on actual event signatures
    if log.topics.len() >= 2 {
        // For now, return a placeholder TransferSingle event
        Some(CtfEventType::TransferSingle(contract::CtfTransferSingle {
            evt_tx_hash: "0x".to_string(),
            evt_index: log.block_index,
            evt_block_time: None,
            evt_block_number: 0,
            operator: log.topics[1].to_vec(),
            from: Vec::new(),
            to: Vec::new(),
            token_id: "0".to_string(),
            value: "0".to_string(),
        }))
    } else {
        None
    }
}

// USDC Event Types
pub enum UsdcEventType {
    Transfer(contract::UsdcTransfer),
    Approval(contract::UsdcApproval),
}

pub fn decode_usdc_events(log: &Log) -> Option<UsdcEventType> {
    // Decode USDC Transfer and Approval events
    if log.topics.len() >= 3 {
        // For now, return a placeholder Transfer event
        Some(UsdcEventType::Transfer(contract::UsdcTransfer {
            evt_tx_hash: "0x".to_string(),
            evt_index: log.block_index,
            evt_block_time: None,
            evt_block_number: 0,
            from: log.topics[1].to_vec(),
            to: log.topics[2].to_vec(),
            value: "0".to_string(),
        }))
    } else {
        None
    }
}

pub fn decode_erc1155_transfer_batch_ctf(log: &Log) -> Option<contract::CtfTransferBatch> {
    if log.topics.len() < 4 {
        return None;
    }
    
    // Decode arrays from log.data
    let mut token_ids = Vec::new();
    let mut values = Vec::new();
    
    // Skip the first 32 bytes (array offset), then read array length
    if log.data.len() >= 64 {
        let array_length = u256_from_bytes(&log.data[32..64]) as usize;
        
        // Read each ID and value (32 bytes each)
        for i in 0..array_length {
            let id_offset = 64 + (i * 32);
            let value_offset = 64 + (array_length * 32) + (i * 32);
            
            if id_offset + 32 <= log.data.len() && value_offset + 32 <= log.data.len() {
                let id = u256_from_bytes(&log.data[id_offset..id_offset + 32]);
                let value = u256_from_bytes(&log.data[value_offset..value_offset + 32]);
                token_ids.push(id.to_string());
                values.push(value.to_string());
            }
        }
    }
    
    Some(contract::CtfTransferBatch {
        evt_tx_hash: "0x".to_string(),
        evt_index: 0,
        evt_block_time: None,
        evt_block_number: 0,
        operator: Vec::new(),
        from: Vec::new(),
        to: Vec::new(),
        token_ids,
        values,
    })
}

pub fn decode_fixed_product_market_maker_creation(log: &Log) -> Option<String> {
    if log.topics.len() < 2 {
        return None;
    }
    
    // Decode the fixedProductMarketMaker address from topics[1]
    let fixed_product_market_maker = format!("0x{}", substreams::Hex(&log.topics[1]).to_string());
    
    Some(fixed_product_market_maker)
}

pub fn decode_uma_merkle_claim(log: &Log) -> Option<contract::DuneRewardClaim> {
    if log.topics.len() < 2 {
        return None;
    }
    
    // Decode the airdropRecipient address from topics[1]
    let airdrop_recipient = format!("0x{}", substreams::Hex(&log.topics[1]).to_string());
    
    // Decode amount from log.data
    let amount = if log.data.len() >= 32 {
        u256_from_bytes(&log.data[0..32]).to_string()
    } else {
        "0".to_string()
    };
    
    // Convert to UMA units (divide by 10^18) as per Dune query
    let amount_uma = if let Ok(val) = amount.parse::<f64>() {
        (val / 1e18).to_string()
    } else {
        "0".to_string()
    };
    
    // For now, use amount as USD amount (would need price data for accurate conversion)
    let usd_amount = amount_uma.clone();
    
    Some(contract::DuneRewardClaim {
        transaction_hash: "0x".to_string(),
        log_index: 0,
        block_timestamp: None,
        block_number: 0,
        airdrop_recipient,
        asset: "uma".to_string(),
        lc_amount: amount_uma,
        usd_amount,
        token_address: "0x".to_string(),
    })
}

pub fn decode_usdc_merkle_claim(log: &Log) -> Option<contract::DuneRewardClaim> {
    if log.topics.len() < 2 {
        return None;
    }
    
    // Decode the airdropRecipient address from topics[1]
    let airdrop_recipient = format!("0x{}", substreams::Hex(&log.topics[1]).to_string());
    
    // Decode amount from log.data
    let amount = if log.data.len() >= 32 {
        u256_from_bytes(&log.data[0..32]).to_string()
    } else {
        "0".to_string()
    };
    
    // Convert to USDC units (divide by 1000000) as per Dune query
    let amount_usdc = if let Ok(val) = amount.parse::<f64>() {
        (val / 1_000_000.0).to_string()
    } else {
        "0".to_string()
    };
    
    Some(contract::DuneRewardClaim {
        transaction_hash: "0x".to_string(),
        log_index: 0,
        block_timestamp: None,
        block_number: 0,
        airdrop_recipient,
        asset: "usdc".to_string(),
        lc_amount: amount_usdc.clone(),
        usd_amount: amount_usdc,
        token_address: "0x".to_string(),
    })
}

pub fn decode_question_initialized(log: &Log) -> Option<QuestionInitializedData> {
    if log.topics.len() < 2 {
        return None;
    }
    
    // Decode questionID from topics[1] (bytes32)
    let question_id = format!("0x{}", substreams::Hex(&log.topics[1]).to_string());
    
    // Decode other parameters from log.data
    // The data structure is: requestTimestamp(uint256), creator(address), ancillaryData(bytes), question(string), rewardToken(address), reward(uint256), proposalBond(uint256)
    let mut offset = 0;
    
    // requestTimestamp (uint256) - 32 bytes
    let request_timestamp = if log.data.len() >= offset + 32 {
        u256_from_bytes(&log.data[offset..offset + 32]).to_string()
    } else {
        "0".to_string()
    };
    offset += 32;
    
    // creator (address) - 32 bytes (padded)
    let creator = if log.data.len() >= offset + 32 {
        format!("0x{}", substreams::Hex(&log.data[offset + 12..offset + 32]).to_string())
    } else {
        "0x".to_string()
    };
    offset += 32;
    
    // ancillaryData (bytes) - dynamic, skip for now
    // question (string) - dynamic, skip for now
    // rewardToken (address) - 32 bytes (padded)
    let reward_token = if log.data.len() >= offset + 32 {
        format!("0x{}", substreams::Hex(&log.data[offset + 12..offset + 32]).to_string())
    } else {
        "0x".to_string()
    };
    offset += 32;
    
    // reward (uint256) - 32 bytes
    let reward = if log.data.len() >= offset + 32 {
        u256_from_bytes(&log.data[offset..offset + 32]).to_string()
    } else {
        "0".to_string()
    };
    offset += 32;
    
    // proposalBond (uint256) - 32 bytes
    let proposal_bond = if log.data.len() >= offset + 32 {
        u256_from_bytes(&log.data[offset..offset + 32]).to_string()
    } else {
        "0".to_string()
    };
    
    // For now, use a placeholder question - in a real implementation, you'd decode the dynamic string
    let question = format!("Question for {}", question_id);
    
    Some(QuestionInitializedData {
        question_id,
        request_timestamp,
        creator,
        ancillary_data: "0x".to_string(),
        question,
        reward_token,
        reward,
        proposal_bond,
    })
}

// Helper struct for QuestionInitialized data
pub struct QuestionInitializedData {
    pub question_id: String,
    pub request_timestamp: String,
    pub creator: String,
    pub ancillary_data: String,
    pub question: String,
    pub reward_token: String,
    pub reward: String,
    pub proposal_bond: String,
}
