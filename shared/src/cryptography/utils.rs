use std::env;

use crate::cryptography::hashing::hash;

pub fn generate_key<'a>() -> String {
    let secret = env::var("encryption_key").expect("No Encryption Key Provided.");
    let key = &hash(secret)[..32];
    return String::from(key);
}
