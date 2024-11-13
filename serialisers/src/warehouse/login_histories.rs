use chrono::Local;
use database::schema::login_histories;
use diesel::{PgConnection, RunQueryDsl, SelectableHelper};

use models::warehouse::login_histories::LoginHistory;
use shared::constants::users::{Country, LoginMethod};
use uuid::Uuid;

pub fn create_login_hostory(db: &mut PgConnection) -> LoginHistory {
    let login = LoginHistory {
        id: Uuid::new_v4().to_string(),
        login_id: Uuid::new_v4().to_string(),
        user_id: Uuid::new_v4().to_string(),
        session_id: Uuid::new_v4().to_string(),
        login_date: Local::now().naive_local(),
        login_location: Country::AFGHANISTAN.to_string().0,
        login_device: String::from("String"),
        login_method: LoginMethod::EMAIL.to_string(),
        logged_in: true,
        logout_date: Local::now().naive_local(),
        authentication_token: Uuid::new_v4().to_string(),
    };
    let login_history = diesel::insert_into(login_histories::table)
        .values(&login)
        .returning(LoginHistory::as_returning())
        .get_result(db)
        .expect("Invalid Login History.");
    return login_history;
}
