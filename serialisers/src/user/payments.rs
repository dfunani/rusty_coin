use std::collections::HashMap;

use chrono::Local;
use database::schema::payments;
use database::schema::payments::dsl::*;
use diesel::{ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl, SelectableHelper};

use models::user::payments::Payment;
use shared::constants::users::{Role, Status};
use shared::cryptography::encoding::encode;
use shared::cryptography::encryption::encrypt;
use shared::cryptography::hashing::hash;
use shared::cryptography::utils::generate_key;
use uuid::Uuid;

use crate::warehouse::cards::create_cards;

pub fn create_payments_profile(db: &mut PgConnection, private_account_id: &str) -> Payment {
    let card = create_cards(db);

    let payment = Payment {
        id: Uuid::new_v4().to_string(),
        payment_id: Uuid::new_v4().to_string(),
        account_id: private_account_id.to_string(),
        card_id: card.card_id,
        name: String::from("New Payment Profile"),
        description: String::from("New Payment Profile Created."),
        balance: 0.0,
        status: Status::NEW.to_string(),
        created_date: Local::now().naive_local(),
        updated_date: Local::now().naive_local(),
    };

    let response = diesel::insert_into(payments::table)
        .values(&payment)
        .returning(Payment::as_returning())
        .get_result(db)
        .expect("Invalid Payment Profile.");
    return response;
}

pub fn read_payments_profile(db: &mut PgConnection, public_payments_id: &str) -> Payment {
    let responses: Vec<Payment> = payments
        .filter(payment_id.eq(public_payments_id))
        .load(db)
        .expect("Invalid Payment ID.");

    if responses.len() != 1 {
        panic!("Invalid Payment ID.");
    }

    let response = responses[0].clone();
    return response;
}

// pub fn update_user(db: &mut PgConnection, private_id: String, data: &mut UpdateUser) -> User {
//     if data.password != None {
//         data.password = Some(hash(data.password.as_ref().unwrap()));
//     };

//     if data.updated_date != None {
//         data.updated_date = Some(Local::now().naive_local());
//     }

//     let update_data = data.clone();
//     let response = diesel::update(users.find(private_id))
//         .set(update_data)
//         .get_result(db)
//         .expect("Invalid ID - User");
//     return response;
// }

// pub fn delete_user(db: &mut PgConnection, private_id: String) -> String {
//     diesel::delete(users.find(private_id.clone()))
//         .returning(User::as_returning())
//         .execute(db)
//         .expect("Invalid ID - User");
//     return String::from(format!("Deleted {}", private_id));
// }
