use chrono::NaiveDateTime;
use database::schema::users;
use diesel::prelude::*;
use std::cmp::{Eq, Ord, PartialEq, PartialOrd};

use crate::Model;

#[derive(Queryable, Insertable, Selectable, Debug, Ord, Eq, PartialOrd, PartialEq, Clone)]
#[diesel(table_name = users)]
pub struct User {
    pub id: String,
    pub user_id: String,
    pub email: String,
    pub password: String,
    pub status: String,
    pub role: String,
    pub salt_value: String,
    pub created_date: NaiveDateTime,
    pub updated_date: NaiveDateTime,
}

#[derive(AsChangeset, Clone, Selectable)]
#[diesel(table_name = users)]
pub struct UpdateUser {
    pub password: Option<String>,
    pub status: Option<String>,
    pub role: Option<String>,
    pub updated_date: Option<NaiveDateTime>,
}

impl Model for User {
    fn to_string(&self) -> String {
        return String::from(format!("User ID: {}", self.user_id.to_string()));
    }
}
