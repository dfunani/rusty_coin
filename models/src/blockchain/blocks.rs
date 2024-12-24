use chrono::NaiveDateTime;
use database::schema::blocks;
use diesel::prelude::*;

use crate::Model;

#[derive(Queryable, Insertable, Selectable, Debug, Clone)]
#[diesel(table_name = blocks)]
pub struct Block {
    pub id: String,
    pub block_id: String,
    pub transaction_id: String,
    pub contract_id: String,
    pub previous_block_id: String,
    pub next_block_id: String,
    pub block_type: String,
    pub created_date: NaiveDateTime,
    pub updated_date: NaiveDateTime,
}

impl Model for Block {
    fn to_string(&self) -> String {
        return String::from(format!("Block ID: {}", self.block_id.to_string()));
    }
}
