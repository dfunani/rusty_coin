// use chrono::{DateTime, Local};
// use uuid::Uuid;

// use crate::{
//     models::model::Model,
//     shared::constants::user::{
//         Communication, DataSharingPreference, ProfileVisibility, Theme, Verification,
//     },
// };

// #[derive(Debug)]
// pub struct SettingsProfile {
//     pub id: Uuid,
//     pub settings_id: Uuid,
//     pub account_id: Uuid,
//     pub email_status: Verification,
//     pub communication_status: Verification,
//     pub mfa_enabled: bool,
//     pub mfa_last_used_date: Option<DateTime<Local>>,
//     pub profile_visibility_preference: ProfileVisibility,
//     pub data_sharing_preferences: &'static [DataSharingPreference],
//     pub communication_preference: Communication,
//     pub location_tracking_enabled: bool,
//     pub cookies_enabled: bool,
//     pub theme_preference: Theme,
//     pub created_date: DateTime<Local>,
//     pub updated_date: DateTime<Local>,
// }

// impl Model for SettingsProfile {
//     fn to_string(&self) -> String {
//         return String::from(format!(
//             "Settings Profile ID: {}",
//             self.settings_id.to_string()
//         ));
//     }
// }
