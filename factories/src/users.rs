use diesel::PgConnection;
use models::Model;
use serialisers::{user::users::create_user, warehouse::login_histories::create_login_hostory};

use std::collections::HashMap;

pub fn generate_user(
    db: &mut PgConnection,
    user_email: String,
    user_password: String,
) -> HashMap<String, Box<dyn Model>> {
    let response = create_user(db, user_email, user_password);

    // let account = Account {
    //     id: Uuid::new_v4(),
    //     account_id: Uuid::new_v4(),
    //     user_id: Uuid::new_v4(),
    //     status: Status::NEW,
    // };

    // let payment = PaymentProfile {
    //     id: Uuid::new_v4(),
    //     payment_id: Uuid::new_v4(),
    //     account_id: Uuid::new_v4(),
    //     card_id: Uuid::new_v4(),
    //     name: String::from("Hello World."),
    //     description: String::from("Goodbye World."),
    //     balance: 12.0,
    //     status: Status::NEW,
    // };

    // let interests = &[];

    // let profile = UserProfile {
    //     id: Uuid::new_v4(),
    //     profile_id: Uuid::new_v4(),
    //     account_id: Uuid::new_v4(),
    //     first_name: String::from("Uuid::new_v4()"),
    //     last_name: String::from("Uuid::new_v4()"),
    //     username: String::from("Uuid::new_v4()"),
    //     date_of_birth: Local::now().date_naive(),
    //     gender: String::from("Uuid::new_v4()"),
    //     profile_picture: String::from("Uuid::new_v4()"),
    //     mobile_number: String::from("Uuid::new_v4()"),
    //     country: String::from("Uuid::new_v4()"),
    //     language: String::from("Uuid::new_v4()"),
    //     biography: String::from("Uuid::new_v4()"),
    //     occupation: String::from("Uuid::new_v4()"),
    //     interests: interests,
    //     social_media_links: HashMap::new(),
    //     status: String::from("Uuid::new_v4()"),
    // };

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

    // let card = Card {
    //     id: Uuid::new_v4(),
    //     card_id: Uuid::new_v4(),
    //     card_number: String::from("String"),
    //     card_type: CardType::CHEQUE,
    //     status: Status::NEW,
    //     pin: String::from("123456"),
    //     salt_value: Uuid::new_v4(),
    //     created_date: Local::now(),
    //     updated_date: Local::now(),
    // };

    let login = create_login_hostory(db);

    let mut dict: HashMap<String, Box<dyn Model>> = HashMap::new();
    dict.insert(String::from("user"), Box::new(response));
    dict.insert(String::from("login"), Box::new(login));

    // dict.insert(String::from("account"), Box::new(account));
    // dict.insert(String::from("payment"), Box::new(payment));
    // dict.insert(String::from("profile"), Box::new(profile));
    // dict.insert(String::from("settings"), Box::new(settings));
    // dict.insert(String::from("card"), Box::new(card));
    return dict;
}
