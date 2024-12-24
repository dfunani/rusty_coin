use std::collections::HashMap;

use chrono::Local;
use database::schema::accounts;
use database::schema::accounts::dsl::*;
use diesel::{ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl, SelectableHelper};

use models::user::accounts::Account;
use shared::constants::users::{Role, Status};
use uuid::Uuid;

pub fn create_account(db: &mut PgConnection, private_user_id: String) -> Account {
    let account = Account {
        id: Uuid::new_v4().to_string(),
        account_id: Uuid::new_v4().to_string(),
        user_id: private_user_id,
        status: Status::NEW.to_string(),
        created_date: Local::now().naive_local(),
        updated_date: Local::now().naive_local(),
    };

    let response = diesel::insert_into(accounts::table)
        .values(&account)
        .returning(Account::as_returning())
        .get_result(db)
        .expect("Invalid User.");
    return response;
}

pub fn read_account(db: &mut PgConnection, public_id: String) -> Account {
    let responses: Vec<Account> = accounts
        .filter(account_id.eq(public_id))
        .load(db)
        .expect("Invalid Account ID.");

    if responses.len() != 1 {
        panic!("Invalid Account ID.");
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
