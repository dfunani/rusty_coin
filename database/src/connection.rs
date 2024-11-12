use std::env;
use diesel::{pg::PgConnection, Connection};

fn build_connection_string() -> String {
    let host = env::var("DB_HOST").expect("Database Host Not Set.");
    let username = env::var("DB_USER").expect("Database Username Not Set");
    let password = env::var("DB_PASSWORD").expect("Database Password Not Set");
    let port = env::var("DB_PORT").expect("Database Port Not Set");
    let db_name = env::var("DB_NAME").expect("Database Name Not Set");
    return format!("postgres://{username}:{password}@{host}:{port}/{db_name}");
}

pub fn db_connection() -> PgConnection {
    let database_url = build_connection_string();
    let connection = PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Invalid DB COnnection URL - {}", database_url));
    return connection;
}