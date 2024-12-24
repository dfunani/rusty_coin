use chrono::Local;
use database::schema::blocks;
use database::schema::blocks::dsl::*;
use diesel::{ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl, SelectableHelper};

use models::blockchain::blocks::Block;
use shared::constants::blockchain::BlockType;
use uuid::Uuid;

pub fn create_block(
    db: &mut PgConnection,
    private_transaction_id: String,
    private_contract_id: String,
) -> Block {
    if private_transaction_id != "" && private_contract_id != "" {
        panic!("Invalid Block")
    }

    let mut blocktype = BlockType::UNIT.to_string();

    if private_contract_id != "" {
        blocktype = BlockType::CONTRACT.to_string();
    } else if private_transaction_id != "" {
        blocktype = BlockType::TRANSACTION.to_string();
    }

    let account = Block {
        id: Uuid::new_v4().to_string(),
        block_id: Uuid::new_v4().to_string(),
        transaction_id: private_transaction_id,
        contract_id: private_contract_id,
        previous_block_id: String::from(""),
        next_block_id: String::from(""),
        block_type: blocktype,
        created_date: Local::now().naive_local(),
        updated_date: Local::now().naive_local(),
    };

    let response = diesel::insert_into(blocks::table)
        .values(&account)
        .returning(Block::as_returning())
        .get_result(db)
        .expect("Invalid User.");
    return response;
}

pub fn read_block(db: &mut PgConnection, public_id: String) -> Block {
    let responses: Vec<Block> = blocks
        .filter(block_id.eq(public_id))
        .load(db)
        .expect("Invalid Block ID.");

    if responses.len() != 1 {
        panic!("Invalid Block ID.");
    }

    let response = responses[0].clone();
    return response;
}