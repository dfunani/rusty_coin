// use uuid::Uuid;

// use crate::{models::model::Model, shared::constants::user::Status};

// #[derive(Debug)]
// pub struct PaymentProfile {
//     pub id: Uuid,
//     pub payment_id: Uuid,
//     pub account_id: Uuid,
//     pub card_id: Uuid,
//     pub name: String,
//     pub description: String,
//     pub status: Status,
//     pub balance: f64,
// }

// impl Model for PaymentProfile{
//     fn to_string(&self) -> String {
//         return String::from(format!("Payment Profile ID: {}", self.payment_id.to_string()));
//     }
// }
