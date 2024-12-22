use chrono::NaiveDateTime;
use database::schema::settings;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::Model;

#[derive(Serialize, Deserialize, Debug)]
pub struct Interests {
    interest: String,
}

#[derive(Queryable, Insertable, Selectable, Debug)]
#[diesel(table_name = settings)]
pub struct Settings {
    pub id: String,
    pub settings_id: String,
    pub account_id: String,
    pub email_status: String,
    pub communication_status: String,
    pub mfa_enabled: String,
    pub mfa_last_used_date: NaiveDateTime,
    pub profile_visibility_preference: String,
    pub data_sharing_preferences: String,
    pub communication_preference: String,
    pub location_tracking_enabled: bool,
    pub cookies_enabled: bool,
    pub theme_preference: String,
    pub created_date: NaiveDateTime,
    pub updated_date: NaiveDateTime,
}

impl Model for Settings {
    fn to_string(&self) -> String {
        return String::from(format!("Account ID: {}", self.account_id.to_string()));
    }
}
