use diesel::PgConnection;
use models::{blockchain::blocks::Block, warehouse::cards::Card, Model};
use serialisers::{
    blockchain::{
        blocks::create_block, contracts::create_contract, transactions::create_transaction,
    },
    user::{
        accounts::{create_account, read_account},
        payments::{create_payments_profile, read_payments_profile},
        profiles::{create_profile, read_profile},
        settings::{create_settings, read_settings},
        users::{create_user, read_user},
    },
    warehouse::{
        cards::{create_cards, read_card},
        login_histories::{create_login_history, read_login_history},
    },
};
use shared::{constants::blockchain::BlockType, utils::helpers::extract_object_id};

use std::collections::HashMap;

pub fn generate_user(
    db: &mut PgConnection,
    user_email: String,
    user_password: String,
) -> HashMap<String, Box<dyn Model>> {
    let mut user = create_user(db, user_email, user_password);
    let mut account = create_account(db, String::from(&user.id));
    let mut payment = create_payments_profile(db, String::from(&account.id));
    let mut profile = create_profile(db, String::from(&account.id));
    let mut settings = create_settings(db, String::from(&account.id));
    let mut login = create_login_history(db);

    user = read_user(db, extract_object_id(&user.to_string()));
    account = read_account(db, extract_object_id(&account.to_string()));
    payment = read_payments_profile(db, extract_object_id(&payment.to_string()));
    profile = read_profile(db, extract_object_id(&profile.to_string()));
    settings = read_settings(db, extract_object_id(&settings.to_string()));
    login = read_login_history(db, extract_object_id(&login.to_string()));
    let card = read_card(db, String::from(&payment.card_id));

    let mut dict: HashMap<String, Box<dyn Model>> = HashMap::new();

    dict.insert(String::from("user"), Box::new(user));
    dict.insert(String::from("account"), Box::new(account));
    dict.insert(String::from("payment"), Box::new(payment));
    dict.insert(String::from("profile"), Box::new(profile));
    dict.insert(String::from("settings"), Box::new(settings));
    dict.insert(String::from("login"), Box::new(login));
    dict.insert(String::from("card"), Box::new(card));

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
