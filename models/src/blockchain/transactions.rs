use chrono::NaiveDateTime;
use database::schema::transactions;
use diesel::prelude::*;

use crate::Model;

#[derive(Queryable, Insertable, Selectable, Debug)]
#[diesel(table_name = transactions)]
pub struct Transaction {
    pub id: String,
    pub transaction_id: String,
    pub sender: String,
    pub receiver: String,
    pub amount: f64,
    pub title: String,
    pub description: String,
    pub sender_signiture: String,
    pub receiver_signiture: String,
    pub transaction_status: String,
    pub salt_value: String,
    pub created_date: NaiveDateTime,
    pub updated_date: NaiveDateTime,
}

impl Model for Transaction {
    fn to_string(&self) -> String {
        return String::from(format!(
            "Transaction ID: {}",
            self.transaction_id.to_string()
        ));
    }
}
