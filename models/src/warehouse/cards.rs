use crate::Model;
use chrono::NaiveDateTime;
use database::schema::cards;
use diesel::prelude::*;
use std::cmp::{Eq, Ord, PartialEq, PartialOrd};

#[derive(Queryable, Insertable, Selectable, Debug, Ord, Eq, PartialOrd, PartialEq, Clone)]
#[diesel(table_name = cards)]
pub struct Card {
    pub id: String,
    pub card_id: String,
    pub card_number: String,
    pub card_type: String,
    pub status: String,
    pub pin: String,
    pub salt_value: String,
    pub created_date: NaiveDateTime,
    pub updated_date: NaiveDateTime,
}

impl Model for Card {
    fn to_string(&self) -> String {
        return String::from(format!("Login history ID: {}", self.card_id.to_string()));
    }
}
