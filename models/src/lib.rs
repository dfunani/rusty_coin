pub mod blockchain;
pub mod user;
pub mod warehouse;

use std::fmt::Debug;

pub trait Model: Debug {
    fn to_string(&self) -> String;
}
