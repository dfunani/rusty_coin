use std::collections::HashMap;

use chrono::Local;
use database::schema::payments;
use database::schema::users::dsl::*;
use diesel::{ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl, SelectableHelper};

use models::user::payments::Payment;
use shared::constants::users::{Role, Status};
use shared::cryptography::encoding::encode;
use shared::cryptography::encryption::encrypt;
use shared::cryptography::hashing::hash;
use shared::cryptography::utils::generate_key;
use uuid::Uuid;

use crate::warehouse::cards::create_cards;

pub fn create_payments_profile(
    db: &mut PgConnection,
    account_id: String,
    name: String,
    description: String,
) -> Payment {
    let card = create_cards(db);

    let payment = Payment {
        id: Uuid::new_v4().to_string(),
        payment_id: Uuid::new_v4().to_string(),
        account_id: account_id,
        card_id: card.id,
        name: name,
        description: description,
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

// pub fn read_user(db: &mut PgConnection, public_id: String) -> User {
//     let responses: Vec<User> = users
//         .filter(user_id.eq(public_id))
//         .load(db)
//         .expect("Invalid User ID.");

//     if responses.len() != 1 {
//         panic!("Invalid User ID.");
//     }

//     let response = responses[0].clone();
//     return response;
// }

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
