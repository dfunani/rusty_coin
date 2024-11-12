// use chrono::{DateTime, Local};
// use uuid::Uuid;

// use crate::{models::model::Model, shared::constants::user::{Country, LoginMethod}};

// #[derive(Debug)]
// pub struct LoginHistory {
//     pub id: Uuid,
//     pub login_id: Uuid,
//     pub user_id: Uuid,
//     pub session_id: Uuid,
//     pub login_date: DateTime<Local>,
//     pub login_location: Country,
//     pub login_device: String,
//     pub login_method: LoginMethod,
//     pub logged_in: bool,
//     pub logout_date: Option<DateTime<Local>>,
//     pub authentication_token: DateTime<Local>,
// }

// impl Model for LoginHistory {
//     fn to_string(&self) -> String {
//         return String::from(format!("Login history ID: {}", self.login_id.to_string()));
//     }
// }
