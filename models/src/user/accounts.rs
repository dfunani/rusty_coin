use chrono::NaiveDateTime;
use database::schema::accounts;
use diesel::prelude::*;

use crate::Model;

#[derive(Queryable, Insertable, Selectable, Debug, Clone)]
#[diesel(table_name = accounts)]
pub struct Account {
    pub id: String,
    pub account_id: String,
    pub user_id: String,
    pub status: String,
    pub created_date: NaiveDateTime,
    pub updated_date: NaiveDateTime,
}

impl Model for Account {
    fn to_string(&self) -> String {
        return String::from(format!("Account ID: {}", self.account_id.to_string()));
    }
}
