use database::connection;
use dotenvy::dotenv;
use factories::users::generate_user;
use models::user::users::UpdateUser;
use serialisers::user::users::{delete_user, read_user, update_user};
use shared::constants::users::{Role, Status};

// #[launch]
fn main() {
    dotenv().ok();
    let db = &mut connection::db_connection();

    let email = String::from("dfunani@live.co.za");
    let password = String::from("password");
    let mut user_object = generate_user(db, email, password);

    println!("{:#?}", &user_object.get_mut("user").unwrap());
    println!("{:#?}", user_object.get_mut("account").unwrap());
    println!("{:#?}", user_object.get_mut("payment").unwrap());
    println!("{:#?}", user_object.get_mut("login").unwrap());
}
