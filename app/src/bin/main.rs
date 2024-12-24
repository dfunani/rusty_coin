use database::connection;
use dotenvy::dotenv;
use factories::users::{generate_block, generate_user};
use models::user::{payments::Payment, users::UpdateUser};
use serialisers::{
    blockchain::{
        blocks::create_block, contracts::create_contract, transactions::create_transaction,
    },
    user::users::{delete_user, read_user, update_user},
};
use shared::{
    constants::users::{Role, Status},
    utils::helpers::extract_object_id,
};

// #[launch]
fn main() {
    dotenv().ok();
    let db = &mut connection::db_connection();

    let email = String::from("dfunani@live.co.za");
    let password = String::from("password");
    let mut user_object = generate_user(db, &email, &password);

    let block_type = shared::constants::blockchain::BlockType::UNIT;
    let pid = user_object.get_mut("payment").unwrap();
    let block = generate_block(db, pid, pid, block_type);

    println!("{:#?}", user_object.get("user").unwrap());
    println!("{:#?}", user_object.get("account").unwrap());
    println!("{:#?}", user_object.get("payment").unwrap());
    println!("{:#?}", user_object.get("profile").unwrap());
    println!("{:#?}", user_object.get("settings").unwrap());
    println!("{:#?}", user_object.get("login").unwrap());
    println!("{:#?}", user_object.get("card").unwrap());
    println!("{:#?}", block);
}
