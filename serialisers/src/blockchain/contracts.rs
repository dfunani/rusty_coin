use chrono::Local;
use database::schema::contracts;
use database::schema::contracts::dsl::*;
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

pub fn read_contract(db: &mut PgConnection, public_id: String) -> Contract {
    let responses: Vec<Contract> = contracts
        .filter(contract_id.eq(public_id))
        .load(db)
        .expect("Invalid Account ID.");

    if responses.len() != 1 {
        panic!("Invalid Account ID.");
    }

    let response = responses[0].clone();
    return response;
}