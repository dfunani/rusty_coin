use std::collections::HashMap;

use chrono::{NaiveDate, NaiveDateTime};
use database::schema::profiles;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::Model;

#[derive(Serialize, Deserialize, Debug)]
pub struct Interests {
    interest: String,
}

#[derive(Queryable, Insertable, Selectable, Debug, Clone)]
#[diesel(table_name = profiles)]
pub struct Profile {
    pub id: String,
    pub profile_id: String,
    pub account_id: String,
    pub first_name: String,
    pub last_name: String,
    pub username: String,
    pub date_of_birth: NaiveDate,
    pub gender: String,
    pub profile_picture: String,
    pub mobile_number: String,
    pub country: String,
    pub language: String,
    pub biography: String,
    pub occupation: String,
    pub interests: String,
    pub social_media_links: String,
    pub status: String,
    pub created_date: NaiveDateTime,
    pub updated_date: NaiveDateTime,
}

// pub struct UpdateProfile {
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
//     pub interests: diesel_json::Json<Interests>,
//     pub social_media_links: diesel_json::Json<Interests>,
//     pub status: String,
//     pub created_date: NaiveDateTime,
//     pub updated_date: NaiveDateTime,
// }

impl Model for Profile {
    fn to_string(&self) -> String {
        return String::from(format!("Profile ID: {}", self.profile_id.to_string()));
    }
}
