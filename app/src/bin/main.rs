use database::connection;
use dotenvy::dotenv;
use factories::users::generate_user;
use models::user::users::UpdateUser;
use serialisers::{
    blockchain::{
        blocks::create_block, contracts::create_contract, transactions::create_transaction,
    },
    user::users::{delete_user, read_user, update_user},
};
use shared::constants::users::{Role, Status};

// #[launch]
fn main() {
    dotenv().ok();
    let db = &mut connection::db_connection();

    let email = String::from("dfunani@live.co.za");
    let password = String::from("password");
    let mut user_object = generate_user(db, email, password);

    let contractor_id = String::from("88dffaad-5d15-4a8d-8c64-8ad28f28dac2");
    let contractee_id = String::from("88dffaad-5d15-4a8d-8c64-8ad28f28dac2");
    let contract = create_contract(db, contractor_id.clone(), contractee_id.clone());
    let transaction = create_transaction(db, contractor_id.clone(), contractee_id.clone());
    let block = create_block(db, contractor_id.clone(), String::from(""));

    println!("{:#?}", user_object.get_mut("user").unwrap());
    println!("{:#?}", user_object.get_mut("account").unwrap());
    println!("{:#?}", user_object.get_mut("payment").unwrap());
    println!("{:#?}", user_object.get_mut("profile").unwrap());
    println!("{:#?}", user_object.get_mut("settings").unwrap());
    println!("{:#?}", user_object.get_mut("login").unwrap());
    println!("{:#?}", contract);
    println!("{:#?}", transaction);
    println!("{:#?}", block);
}
