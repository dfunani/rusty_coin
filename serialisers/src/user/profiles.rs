use std::collections::HashMap;

use chrono::Local;
use database::schema::profiles;
use database::schema::profiles::dsl::*;
use diesel::{ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl, SelectableHelper};

use models::user::profiles::Profile;
use shared::constants::users::{Role, Status};
use uuid::Uuid;

pub fn create_profile(db: &mut PgConnection, public_account_id: String) -> Profile {
    let interest: Vec<String> = vec![];
    let socials: HashMap<String, String> = HashMap::new();
    let profile = Profile {
        id: Uuid::new_v4().to_string(),
        profile_id: Uuid::new_v4().to_string(),
        account_id: public_account_id,
        first_name: String::from(""),
        last_name: String::from(""),
        username: String::from(""),
        date_of_birth: Local::now().date_naive(),
        gender: String::from(""),
        profile_picture: String::from(""),
        mobile_number: String::from(""),
        country: String::from(""),
        language: String::from(""),
        biography: String::from(""),
        occupation: String::from(""),
        interests: serde_json::to_string(&interest).expect("Invalid Interests"),
        social_media_links: serde_json::to_string(&socials).expect("Invalid Social Media Links"),
        status: Status::NEW.to_string(),
        created_date: Local::now().naive_local(),
        updated_date: Local::now().naive_local(),
    };

    let response = diesel::insert_into(profiles::table)
        .values(&profile)
        .returning(Profile::as_returning())
        .get_result(db)
        .expect("Invalid User.");
    return response;
}

pub fn read_profile(db: &mut PgConnection, public_id: String) -> Profile {
    let responses: Vec<Profile> = profiles
        .filter(profile_id.eq(public_id))
        .load(db)
        .expect("Invalid User ID.");

    if responses.len() != 1 {
        panic!("Invalid User ID.");
    }

    let response = responses[0].clone();
    return response;
}

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
