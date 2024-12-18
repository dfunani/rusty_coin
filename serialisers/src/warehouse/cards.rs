use chrono::Local;
use database::schema::cards;
use diesel::{PgConnection, RunQueryDsl, SelectableHelper};

use models::warehouse::{cards::Card, login_histories::LoginHistory};
use shared::constants::users::{CardType, Country, LoginMethod, Status};
use uuid::Uuid;

pub fn create_cards(db: &mut PgConnection) -> Card {
    let cards = Card {
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
        .values(&cards)
        .returning(Card::as_returning())
        .get_result(db)
        .expect("Invalid Card.");
    return card;
}
