use diesel::PgConnection;
use models::{blockchain::blocks::Block, warehouse::cards::Card, Model};
use serialisers::{
    blockchain::{
        blocks::create_block, contracts::create_contract, transactions::create_transaction,
    },
    user::{
        accounts::create_account, payments::create_payments_profile, profiles::create_profile,
        settings::create_settings, users::create_user,
    },
    warehouse::{cards::create_cards, login_histories::create_login_history},
};
use shared::constants::blockchain::BlockType;

use std::collections::HashMap;

pub fn generate_user(
    db: &mut PgConnection,
    user_email: String,
    user_password: String,
) -> HashMap<String, Box<dyn Model>> {
    let user = create_user(db, user_email, user_password);
    let account = create_account(db, String::from(&user.id));
    let payment = create_payments_profile(db, String::from(&account.id));
    let profile = create_profile(db, String::from(&account.id));
    let settings = create_settings(db, String::from(&account.id));
    let login = create_login_history(db);

    let mut dict: HashMap<String, Box<dyn Model>> = HashMap::new();

    dict.insert(String::from("user"), Box::new(user));
    dict.insert(String::from("account"), Box::new(account));
    dict.insert(String::from("payment"), Box::new(payment));
    dict.insert(String::from("profile"), Box::new(profile));
    dict.insert(String::from("settings"), Box::new(settings));
    dict.insert(String::from("login"), Box::new(login));

    return dict;
}

pub fn generate_block(
    db: &mut PgConnection,
    party_a_payment_id: String,
    party_b_payment_id: String,
    block_type: BlockType,
) -> Block {
    match block_type {
        BlockType::CONTRACT => {
            let response = create_contract(db, party_a_payment_id, party_b_payment_id);
            return create_block(db, String::from(""), response.id);
        }
        BlockType::TRANSACTION => {
            let response = create_transaction(db, party_a_payment_id, party_b_payment_id);
            return create_block(db, response.id, String::from(""));
        }
        BlockType::UNIT => {
            return create_block(db, String::from(""), String::from(""));
        }
    }
}
