use crate::pb::contract::v1 as contract;
use substreams::Hex;
use substreams_ethereum::pb::eth::v2::Log;

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
    if log.topics.len() >= 4 {
        Some(contract::OrderFilled {
            evt_tx_hash: "0x".to_string(),
            evt_index: log.block_index,
            evt_block_time: None,
            evt_block_number: 0,
            maker: log.topics[1].to_vec(),
            taker: log.topics[2].to_vec(),
            maker_asset_id: "0".to_string(), // Would need to decode from data
            taker_asset_id: "0".to_string(), // Would need to decode from data
            maker_amount_filled: "0".to_string(), // Would need to decode from data
            taker_amount_filled: "0".to_string(), // Would need to decode from data
            fee: "0".to_string(), // Would need to decode from data
            order_hash: log.topics[3].to_vec(),
        })
    } else {
        None
    }
}

pub fn decode_erc1155_transfer_single(log: &Log) -> Option<contract::Erc1155TransferSingle> {
    // Decode ERC1155 TransferSingle event
    // Event signature: TransferSingle(address indexed operator, address indexed from, address indexed to, uint256 id, uint256 value)
    if log.topics.len() >= 4 {
        Some(contract::Erc1155TransferSingle {
            evt_tx_hash: "0x".to_string(),
            evt_index: log.block_index,
            evt_block_time: None,
            evt_block_number: 0,
            contract_address: log.address.to_vec(),
            operator: log.topics[1].to_vec(),
            from: log.topics[2].to_vec(),
            to: log.topics[3].to_vec(),
            id: "0".to_string(), // Would need to decode from data
            value: "0".to_string(), // Would need to decode from data
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
    if log.topics.len() >= 3 {
        Some(contract::Erc20Transfer {
            evt_tx_hash: "0x".to_string(),
            evt_index: log.block_index,
            evt_block_time: None,
            evt_block_number: 0,
            contract_address: log.address.to_vec(),
            from: log.topics[1].to_vec(),
            to: log.topics[2].to_vec(),
            value: "0".to_string(), // Would need to decode from data
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
