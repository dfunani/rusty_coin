use chrono::Local;
use database::schema::transactions;
use database::schema::transactions::dsl::*;
use diesel::{ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl, SelectableHelper};

use models::blockchain::transactions::Transaction;
use shared::constants::blockchain::{BlockType, TransactionStatus};
use uuid::Uuid;

pub fn create_transaction(
    db: &mut PgConnection,
    sender_id: String,
    receiver_id: String,
) -> Transaction {
    let account = Transaction {
        id: Uuid::new_v4().to_string(),
        transaction_id: Uuid::new_v4().to_string(),
        sender: sender_id,
        receiver: receiver_id,
        title: String::from("New Contract"),
        description: String::from("New Contract Created"),
        amount: 0.0,
        transaction_status: TransactionStatus::DRAFT.to_string(),
        sender_signiture: String::from(""),
        receiver_signiture: String::from(""),
        salt_value: Uuid::new_v4().to_string(),
        created_date: Local::now().naive_local(),
        updated_date: Local::now().naive_local(),
    };

    let response = diesel::insert_into(transactions::table)
        .values(&account)
        .returning(Transaction::as_returning())
        .get_result(db)
        .expect("Invalid User.");
    return response;
}

pub fn read_transaction(db: &mut PgConnection, public_id: String) -> Transaction {
    let responses: Vec<Transaction> = transactions
        .filter(transaction_id.eq(public_id))
        .load(db)
        .expect("Invalid Account ID.");

    if responses.len() != 1 {
        panic!("Invalid Account ID.");
    }

    let response = responses[0].clone();
    return response;
}
