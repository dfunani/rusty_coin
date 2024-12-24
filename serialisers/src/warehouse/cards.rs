use chrono::Local;
use database::schema::cards;
use database::schema::cards::dsl::*;
use diesel::{ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl, SelectableHelper};

use models::warehouse::cards::Card;
use shared::constants::users::{CardType, Country, LoginMethod, Status};
use uuid::Uuid;

pub fn create_cards(db: &mut PgConnection) -> Card {
    let card = Card {
        id: Uuid::new_v4().to_string(),
        card_id: Uuid::new_v4().to_string(),
        card_number: String::from("1234567890"),
        card_type: CardType::CHEQUE.to_string().0,
        status: Status::NEW.to_string(),
        pin: String::from("123456"),
        salt_value: Uuid::new_v4().to_string(),
        created_date: Local::now().naive_local(),
        updated_date: Local::now().naive_local(),
    };
    let card = diesel::insert_into(cards::table)
        .values(&card)
        .returning(Card::as_returning())
        .get_result(db)
        .expect("Invalid Card.");
    return card;
}

pub fn read_card(db: &mut PgConnection, public_id: String) -> Card {
    let responses: Vec<Card> = cards
        .filter(card_id.eq(&public_id))
        .load(db)
        .expect("Invalid Card ID.");

    println!("{:#?}", public_id.as_str());
    if responses.len() != 1 {
        panic!("Invalid Card ID.");
    }

    let response = responses[0].clone();
    return response;
}
