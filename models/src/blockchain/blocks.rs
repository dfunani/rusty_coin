// use chrono::{DateTime, Local};
// use uuid::Uuid;

// use crate::{models::model::Model, shared::constants::blockchain::BlockType};

// #[derive(Debug)]
// pub struct Block {
//     id: Uuid,
//     block_id: Uuid,
//     transaction_id: Uuid,
//     contract_id: Uuid,
//     previous_block_id: Uuid,
//     next_block_id: Uuid,
//     block_type: BlockType,
//     created_date: DateTime<Local>,
//     updated_date: DateTime<Local>,
// }

// impl Model for Block {
//     fn to_string(&self) -> String {
//         return String::from(format!("Block ID: {}", self.block_id.to_string()));
//     }
// }
