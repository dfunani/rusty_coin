use chrono::NaiveDateTime;
use database::schema::contracts;
use diesel::prelude::*;

use crate::Model;

#[derive(Queryable, Insertable, Selectable, Debug, Clone)]
#[diesel(table_name = contracts)]
pub struct Contract {
    pub id: String,
    pub contract_id: String,
    pub contractor: String,
    pub contractee: String,
    pub title: String,
    pub description: String,
    pub contract: String,
    pub contract_status: String,
    pub contractor_signiture: String,
    pub contractee_signiture: String,
    pub salt_value: String,
    pub created_date: NaiveDateTime,
    pub updated_date: NaiveDateTime,
}

impl Model for Contract {
    fn to_string(&self) -> String {
        return String::from(format!("Contract ID: {}", self.contract_id.to_string()));
    }
}
