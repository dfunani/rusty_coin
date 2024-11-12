// use chrono::{DateTime, Local};
// use uuid::Uuid;

// use crate::{models::model::Model, shared::constants::blockchain::ContractStatus};

// #[derive(Debug)]
// pub struct Contract {
//     id: Uuid,
//     contract_id: Uuid,
//     contractor: Uuid,
//     contractee: Uuid,
//     title: String,
//     description: String,
//     contract: String,
//     contract_status: ContractStatus,
//     contractor_signiture: String,
//     contractee_signiture: String,
//     salt_value: Uuid,
//     created_date: DateTime<Local>,
//     updated_date: DateTime<Local>,
// }

// impl Model for Contract {
//     fn to_string(&self) -> String {
//         return String::from(format!("Contract ID: {}", self.contract_id.to_string()));
//     }
// }
