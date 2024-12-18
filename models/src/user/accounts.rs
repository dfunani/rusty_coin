use database::schema::accounts;
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
#[diesel(table_name = accounts)]
pub struct Account {
    pub id: String,
    pub account_id: String,
    pub user_id: String,
    pub status: String,
}

#[derive(AsChangeset, Clone, Selectable)]
#[diesel(table_name = accounts)]
pub struct UpdateAccount {
    pub id: String,
    pub account_id: String,
    pub user_id: String,
    pub status: String,
}

impl Model for Account {
    fn to_string(&self) -> String {
        return String::from(format!("Account ID: {}", self.account_id.to_string()));
    }
}
