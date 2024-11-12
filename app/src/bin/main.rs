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

    let user_data = user_object.get_mut("user").unwrap();
    println!("Created User: {:#?}", &user_data);

    let mut user = read_user(db, user_data.e);
    println!("Read User: {:#?}", user);

    let private_id = String::from("8665da1a-a10f-4462-9eb0-5d48f2232f0d");
    let mut data = UpdateUser {
        password: Some(String::from("new")),
        status: Some(Status::ACTIVE.to_string()),
        role: Some(Role::DEVELOPER.to_string()),
    };
    user = update_user(db, private_id.clone(), &mut data);
    println!("Updated User: {:#?}", user);

    let result = delete_user(db, private_id);
    println!("Deleted {:#?}", result);

    // println!("{:#?}", user_object.get_mut("account").unwrap());
    // println!("{:#?}", user_object.get_mut("payment").unwrap());
    // println!("{:#?}", user_object.get_mut("profile").unwrap());
    // println!("{:#?}", user_object.get_mut("settings").unwrap());
    // println!("{:#?}", user_object.get_mut("card").unwrap());
    // println!("{:#?}", user_object.get_mut("login").unwrap());
}
