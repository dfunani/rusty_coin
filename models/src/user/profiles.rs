// use database::schema::profiles;
// use diesel::prelude::*;
// use rocket::serde::{Deserialize, Serialize};
// use std::cmp::{Eq, Ord, PartialEq, PartialOrd};

// use crate::Model;

// #[derive(
//     Queryable,
//     Insertable,
//     Selectable,
//     Debug,
//     Ord,
//     Eq,
//     PartialOrd,
//     PartialEq,
//     Serialize,
//     Deserialize,
//     Clone,
// )]
// #[diesel(table_name = profiles)]
// pub struct Account {
//     pub id: String,
//     pub profile_id: String,
//     pub account_id: String,
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

// #[derive(AsChangeset, Clone, Selectable)]
// #[diesel(table_name = accounts)]
// pub struct UpdateAccount {
//     pub id: String,
//     pub profile_id: String,
//     pub account_id: String,
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

// impl Model for Account {
//     fn to_string(&self) -> String {
//         return String::from(format!("Account ID: {}", self.account_id.to_string()));
//     }
// }
