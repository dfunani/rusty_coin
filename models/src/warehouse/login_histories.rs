use crate::Model;
use chrono::NaiveDateTime;
use database::schema::login_histories;
use diesel::prelude::*;
use std::cmp::{Eq, Ord, PartialEq, PartialOrd};

#[derive(Queryable, Insertable, Selectable, Debug, Clone)]
#[diesel(table_name = login_histories)]
pub struct LoginHistory {
    pub id: String,
    pub login_id: String,
    pub user_id: String,
    pub session_id: String,
    pub login_date: NaiveDateTime,
    pub login_location: String,
    pub login_device: String,
    pub login_method: String,
    pub logged_in: bool,
    pub logout_date: NaiveDateTime,
    pub authentication_token: String,
    pub updated_date: NaiveDateTime,
    pub created_date: NaiveDateTime,
}

impl Model for LoginHistory {
    fn to_string(&self) -> String {
        return String::from(format!("Login history ID: {}", self.login_id.to_string()));
    }
}
