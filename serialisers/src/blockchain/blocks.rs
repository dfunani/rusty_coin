use chrono::Local;
use database::schema::blocks;
use database::schema::users::dsl::*;
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

    let mut block_type = BlockType::UNIT.to_string();

    if private_contract_id != "" {
        block_type = BlockType::CONTRACT.to_string();
    } else if private_transaction_id != "" {
        block_type = BlockType::TRANSACTION.to_string();
    }

    let account = Block {
        id: Uuid::new_v4().to_string(),
        block_id: Uuid::new_v4().to_string(),
        transaction_id: private_transaction_id,
        contract_id: private_contract_id,
        previous_block_id: String::from(""),
        next_block_id: String::from(""),
        block_type,
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

// pub fn read_user(db: &mut PgConnection, public_id: String) -> User {
//     let responses: Vec<User> = users
//         .filter(user_id.eq(public_id))
//         .load(db)
//         .expect("Invalid User ID.");

//     if responses.len() != 1 {
//         panic!("Invalid User ID.");
//     }

//     let response = responses[0].clone();
//     return response;
// }

// pub fn update_user(db: &mut PgConnection, private_id: String, data: &mut UpdateUser) -> User {
//     if data.password != None {
//         data.password = Some(hash(data.password.as_ref().unwrap()));
//     };

//     if data.updated_date != None {
//         data.updated_date = Some(Local::now().naive_local());
//     }

//     let update_data = data.clone();
//     let response = diesel::update(users.find(private_id))
//         .set(update_data)
//         .get_result(db)
//         .expect("Invalid ID - User");
//     return response;
// }

// pub fn delete_user(db: &mut PgConnection, private_id: String) -> String {
//     diesel::delete(users.find(private_id.clone()))
//         .returning(User::as_returning())
//         .execute(db)
//         .expect("Invalid ID - User");
//     return String::from(format!("Deleted {}", private_id));
// }
