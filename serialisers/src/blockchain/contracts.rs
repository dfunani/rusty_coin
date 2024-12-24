use chrono::Local;
use database::schema::contracts;
use database::schema::users::dsl::*;
use diesel::{ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl, SelectableHelper};

use models::blockchain::contracts::Contract;
use shared::constants::blockchain::{BlockType, ContractStatus};
use uuid::Uuid;

pub fn create_contract(
    db: &mut PgConnection,
    contractor_id: String,
    contractee_id: String,
) -> Contract {
    let account = Contract {
        id: Uuid::new_v4().to_string(),
        contract_id: Uuid::new_v4().to_string(),
        contractor: contractor_id,
        contractee: contractee_id,
        title: String::from("New Contract"),
        description: String::from("New Contract Created"),
        contract: String::from(""),
        contract_status: ContractStatus::DRAFT.to_string(),
        contractor_signiture: String::from(""),
        contractee_signiture: String::from(""),
        salt_value: Uuid::new_v4().to_string(),
        created_date: Local::now().naive_local(),
        updated_date: Local::now().naive_local(),
    };

    let response = diesel::insert_into(contracts::table)
        .values(&account)
        .returning(Contract::as_returning())
        .get_result(db)
        .expect("Invalid User.");
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
