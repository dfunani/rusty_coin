use database::schema::settings;
use diesel::prelude::*;
use rocket::serde::{Deserialize, Serialize};
use std::cmp::{Eq, Ord, PartialEq, PartialOrd};

use crate::Model;

#[derive(
    Queryable,
    Insertable,
    Selectable,
    Debug,
    Ord,
    Eq,
    PartialOrd,
    PartialEq,
    Serialize,
    Deserialize,
    Clone,
)]
#[diesel(table_name = settings)]
pub struct Account {
    pub id: Uuid,
    pub settings_id: Uuid,
    pub account_id: Uuid,
    pub email_status: Verification,
    pub communication_status: Verification,
    pub mfa_enabled: bool,
    pub mfa_last_used_date: Option<DateTime<Local>>,
    pub profile_visibility_preference: ProfileVisibility,
    pub data_sharing_preferences: &'static [DataSharingPreference],
    pub communication_preference: Communication,
    pub location_tracking_enabled: bool,
    pub cookies_enabled: bool,
    pub theme_preference: Theme,
    pub created_date: DateTime<Local>,
    pub updated_date: DateTime<Local>,
}

#[derive(AsChangeset, Clone, Selectable)]
#[diesel(table_name = accounts)]
pub struct UpdateAccount {
    pub id: Uuid,
    pub settings_id: Uuid,
    pub account_id: Uuid,
    pub email_status: Verification,
    pub communication_status: Verification,
    pub mfa_enabled: bool,
    pub mfa_last_used_date: Option<DateTime<Local>>,
    pub profile_visibility_preference: ProfileVisibility,
    pub data_sharing_preferences: &'static [DataSharingPreference],
    pub communication_preference: Communication,
    pub location_tracking_enabled: bool,
    pub cookies_enabled: bool,
    pub theme_preference: Theme,
    pub created_date: DateTime<Local>,
    pub updated_date: DateTime<Local>,
}

impl Model for Account {
    fn to_string(&self) -> String {
        return String::from(format!("Account ID: {}", self.account_id.to_string()));
    }
}
