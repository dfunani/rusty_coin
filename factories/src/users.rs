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
    user_email: &str,
    user_password: &str,
) -> HashMap<&'static str, String> {
    let mut user = create_user(db, user_email, user_password);
    let mut account = create_account(db, &user.id);
    let mut payment = create_payments_profile(db, &account.id);
    let mut profile = create_profile(db, &account.id);
    let mut settings = create_settings(db, &account.id);
    let mut login = create_login_history(db, &user.id);

    user = read_user(db, extract_object_id(&user.to_string()));
    account = read_account(db, extract_object_id(&account.to_string()));
    payment = read_payments_profile(db, extract_object_id(&payment.to_string()));
    profile = read_profile(db, extract_object_id(&profile.to_string()));
    settings = read_settings(db, extract_object_id(&settings.to_string()));
    login = read_login_history(db, extract_object_id(&login.to_string()));
    let card = read_card(db, &payment.card_id);

    let mut dict: HashMap<&str, String> = HashMap::new();

    dict.insert("user", user.id);
    dict.insert("account", account.id);
    dict.insert("payment", payment.id);
    dict.insert("profile", profile.id);
    dict.insert("settings", settings.id);
    dict.insert("login", login.id);
    dict.insert("card", card.id);

    return dict;
}

pub fn generate_block(
    db: &mut PgConnection,
    party_a_payment_id: &str,
    party_b_payment_id: &str,
    block_type: BlockType,
) -> Block {
    match block_type {
        BlockType::CONTRACT => {
            let response = create_contract(db, party_a_payment_id, party_b_payment_id);
            return create_block(db, "", &response.id);
        }
        BlockType::TRANSACTION => {
            let response = create_transaction(db, party_a_payment_id, party_b_payment_id);
            return create_block(db, &response.id, "");
        }
        BlockType::UNIT => {
            return create_block(db, "", "");
        }
    }
}
