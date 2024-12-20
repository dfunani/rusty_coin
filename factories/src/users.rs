use diesel::PgConnection;
use models::{warehouse::cards::Card, Model};
use serialisers::{
    user::{accounts::create_account, payments::create_payments_profile, users::create_user},
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
    let payment = create_payments_profile(
        db,
        String::from(&account.id),
        String::from("Payment Profile"),
        String::from("Delali's Expenses Payment Profile"),
    );
    let profile;

    

    // let data_sharing_preferences = &[];
    // let settings = SettingsProfile {
    //     id: Uuid::new_v4(),
    //     settings_id: Uuid::new_v4(),
    //     account_id: Uuid::new_v4(),
    //     email_status: Verification::UNVERIFIED,
    //     communication_status: Verification::UNVERIFIED,
    //     mfa_enabled: false,
    //     mfa_last_used_date: None,
    //     profile_visibility_preference: ProfileVisibility::PUBLIC,
    //     data_sharing_preferences: data_sharing_preferences,
    //     communication_preference: Communication::EMAIL,
    //     location_tracking_enabled: false,
    //     cookies_enabled: false,
    //     theme_preference: Theme::LIGHT,
    //     created_date: Local::now(),
    //     updated_date: Local::now(),
    // };

    let login = create_login_history(db);

    let mut dict: HashMap<String, Box<dyn Model>> = HashMap::new();
    dict.insert(String::from("user"), Box::new(user));
    dict.insert(String::from("account"), Box::new(account));
    dict.insert(String::from("payment"), Box::new(payment));
    dict.insert(String::from("profile"), Box::new(profile));
    dict.insert(String::from("login"), Box::new(login));

    return dict;
}
