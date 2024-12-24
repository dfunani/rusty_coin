use database::connection;
use dotenvy::dotenv;
use factories::users::{generate_block, generate_user};
use models::user::users::UpdateUser;
use serialisers::{
    blockchain::{
        blocks::create_block, contracts::create_contract, transactions::create_transaction,
    },
    user::users::{delete_user, read_user, update_user},
};
use shared::{constants::users::{Role, Status}, utils::helpers::extract_object_id};

// #[launch]
fn main() {    
    dotenv().ok();
    let db = &mut connection::db_connection();

    let email = String::from("dfunani@live.co.za");
    let password = String::from("password");
    let mut user_object = generate_user(db, email, password);

    let party_a_payment_id = String::from("88dffaad-5d15-4a8d-8c64-8ad28f28dac2");
    let party_b_payment_id = String::from("88dffaad-5d15-4a8d-8c64-8ad28f28dac2");
    let block_type = shared::constants::blockchain::BlockType::CONTRACT;
    let block = generate_block(db, party_a_payment_id, party_b_payment_id, block_type);

    println!("{:#?}", user_object.get_mut("user").unwrap());
    println!("{:#?}", user_object.get_mut("account").unwrap());
    println!("{:#?}", user_object.get_mut("payment").unwrap());
    println!("{:#?}", user_object.get_mut("profile").unwrap());
    println!("{:#?}", user_object.get_mut("settings").unwrap());
    println!("{:#?}", user_object.get_mut("login").unwrap());
    println!("{:#?}", user_object.get_mut("card").unwrap());
    println!("{:#?}", block);
}
