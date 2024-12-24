use std::collections::HashMap;

use chrono::Local;
use database::schema::settings;
use database::schema::settings::dsl::*;
use diesel::{ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl, SelectableHelper};

use models::user::settings::Settings;
use models::user::users::{UpdateUser, User};
use shared::constants::users::{Communication, Role, Status, Theme};
use shared::cryptography::encoding::encode;
use shared::cryptography::encryption::encrypt;
use shared::cryptography::hashing::hash;
use shared::cryptography::utils::generate_key;
use uuid::Uuid;

pub fn create_settings(db: &mut PgConnection, private_account_id: &str) -> Settings {
    let data_sharing: Vec<String> = vec![];
    let setting = Settings {
        id: Uuid::new_v4().to_string(),
        settings_id: Uuid::new_v4().to_string(),
        account_id: private_account_id.to_string(),
        email_status: String::from(""),
        communication_status: String::from(""),
        mfa_enabled: String::from(""),
        mfa_last_used_date: Local::now().naive_local(),
        profile_visibility_preference: String::from(""),
        data_sharing_preferences: serde_json::to_string(&data_sharing)
            .expect("Invalid Data Sharing Preferences"),
        communication_preference: Communication::EMAIL.to_string(),
        location_tracking_enabled: false,
        cookies_enabled: false,
        theme_preference: Theme::LIGHT.to_string(),
        created_date: Local::now().naive_local(),
        updated_date: Local::now().naive_local(),
    };

    let response = diesel::insert_into(settings::table)
        .values(&setting)
        .returning(Settings::as_returning())
        .get_result(db)
        .expect("Invalid User.");
    return response;
}

pub fn read_settings(db: &mut PgConnection, public_settings_id: &str) -> Settings {
    let responses: Vec<Settings> = settings
        .filter(settings_id.eq(public_settings_id))
        .load(db)
        .expect("Invalid User ID.");

    if responses.len() != 1 {
        panic!("Invalid User ID.");
    }

    let response = responses[0].clone();
    return response;
}
