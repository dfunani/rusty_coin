// use chrono::{DateTime, Local};
// use uuid::Uuid;

// use crate::{models::model::Model, shared::constants::blockchain::TransactionStatus};

// #[derive(Debug)]
// pub struct Transaction {
//     id: Uuid,
//     transaction_id: Uuid,
//     sender: Uuid,
//     receiver: Uuid,
//     amount: f64,
//     title: String,
//     description: String,
//     sender_signiture: String,
//     receiver_signiture: String,
//     transaction_status: TransactionStatus,
//     salt_value: Uuid,
//     created_date: DateTime<Local>,
//     updated_date: DateTime<Local>,
// }

// impl Model for Transaction {
//     fn to_string(&self) -> String {
//         return String::from(format!(
//             "Transaction ID: {}",
//             self.transaction_id.to_string()
//         ));
//     }
// }
