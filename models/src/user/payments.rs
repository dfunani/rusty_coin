use database::schema::payments;
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
#[diesel(table_name = payments)]
pub struct Payment {
    pub id: String,
    pub payment_id: String,
    pub account_id: String,
    pub card_id: String,
    pub name: String,
    pub description: String,
    pub status: String,
    pub balance: String,
}

#[derive(AsChangeset, Clone, Selectable)]
#[diesel(table_name = payments)]
pub struct UpdatePayment {
    pub id: String,
    pub payment_id: String,
    pub account_id: String,
    pub card_id: String,
    pub name: String,
    pub description: String,
    pub status: String,
    pub balance: String,
}

impl Model for Payment {
    fn to_string(&self) -> String {
        return String::from(format!("Payment ID: {}", self.account_id.to_string()));
    }
}
