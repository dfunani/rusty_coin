// use uuid::Uuid;

// use crate::{models::model::Model, shared::constants::user::Status};

// #[derive(Debug)]
// pub struct Account {
//     pub id: Uuid,
//     pub account_id: Uuid,
//     pub user_id: Uuid,
//     pub status: Status,
// }

// impl Model for Account{
//     fn to_string(&self) -> String {
//         return String::from(format!("Account ID: {}", self.account_id));
//     }
// }