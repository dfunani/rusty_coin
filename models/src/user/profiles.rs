// use chrono::NaiveDate;
// use std::collections::HashMap;
// use uuid::Uuid;

// use crate::{models::model::Model, shared::constants::user::Interest};

// #[derive(Debug)]
// pub struct UserProfile {
//     pub id: Uuid,
//     pub profile_id: Uuid,
//     pub account_id: Uuid,
//     pub first_name: String,
//     pub last_name: String,
//     pub username: String,
//     pub date_of_birth: NaiveDate,
//     pub gender: String,
//     pub profile_picture: String,
//     pub mobile_number: String,
//     pub country: String,
//     pub language: String,
//     pub biography: String,
//     pub occupation: String,
//     pub interests: &'static [Interest],
//     pub social_media_links: HashMap<String, String>,
//     pub status: String,
// }

// impl Model for UserProfile {
//     fn to_string(&self) -> String {
//         return String::from(format!("User Profile ID: {}", self.profile_id.to_string()));
//     }
// }
