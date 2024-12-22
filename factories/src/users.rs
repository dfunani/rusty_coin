use diesel::PgConnection;
use models::{warehouse::cards::Card, Model};
use serialisers::{
    user::{
        accounts::create_account, payments::create_payments_profile, profiles::create_profile, settings::create_settings, users::create_user
    },
    warehouse::{cards::create_cards, login_histories::create_login_history},
};

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
