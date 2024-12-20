use std::{any::Any, collections::HashMap};

use chrono::Local;
use database::schema::profiles::{self, social_media_links};
use database::schema::users::dsl::*;
use diesel::{
    sql_types::Json, ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl, SelectableHelper,
};

use models::user::profiles::Profile;
use shared::constants::users::Status;
use shared::cryptography::hashing::hash;
use uuid::Uuid;

pub fn create_profile(
    db: &mut PgConnection,
    account_id: String,
    profile: HashMap<String, String>,
    interests: Json<Intersests>,
    social_media: Json<Intersests>,
) -> Profile {
    let profile = Profile {
        id: Uuid::new_v4().to_string(),
        profile_id: Uuid::new_v4().to_string(),
        account_id: account_id,
        first_name: String::from("Uuid::new_v4()"),
        last_name: String::from("Uuid::new_v4()"),
        username: String::from("Uuid::new_v4()"),
        date_of_birth: Local::now().date_naive(),
        gender: String::from("Uuid::new_v4()"),
        profile_picture: String::from("Uuid::new_v4()"),
        mobile_number: String::from("Uuid::new_v4()"),
        country: String::from("Uuid::new_v4()"),
        language: String::from("Uuid::new_v4()"),
        biography: String::from("Uuid::new_v4()"),
        occupation: String::from("Uuid::new_v4()"),
        interests: interests,
        social_media_links: social_media,
        status: Status::NEW.to_string(),
    };

    let response = diesel::insert_into(profiles::table)
        .values(&profile)
        .returning(Profile::as_returning())
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
