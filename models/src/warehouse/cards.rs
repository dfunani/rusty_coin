// use chrono::{DateTime, Local};
// use uuid::Uuid;

// use crate::{models::model::Model, shared::constants::user::{CardType, Status}};

// #[derive(Debug)]
// pub struct Card {
//     pub id: Uuid,
//     pub card_id: Uuid,
//     pub card_number: String,
//     pub card_type: CardType,
//     pub status: Status,
//     pub pin: String,
//     pub salt_value: Uuid,
//     pub created_date: DateTime<Local>,
//     pub updated_date: DateTime<Local>,
// }

// impl Model for Card {
//     fn to_string(&self) -> String {
//         return String::from(format!("Card ID: {}", self.card_id.to_string()));
//     }
// }
