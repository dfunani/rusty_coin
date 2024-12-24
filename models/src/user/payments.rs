use chrono::NaiveDateTime;
use database::schema::payments;
use diesel::prelude::*;

use crate::Model;

#[derive(Queryable, Insertable, Selectable, Debug, Clone)]
#[diesel(table_name = payments)]
pub struct Payment {
    pub id: String,
    pub payment_id: String,
    pub account_id: String,
    pub card_id: String,
    pub name: String,
    pub description: String,
    pub status: String,
    pub balance: f64,
    pub created_date: NaiveDateTime,
    pub updated_date: NaiveDateTime,
}

impl Model for Payment {
    fn to_string(&self) -> String {
        return String::from(format!("Payment ID: {}", self.payment_id.to_string()));
    }
}
