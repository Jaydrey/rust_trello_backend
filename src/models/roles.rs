use std::fmt::Display;
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Role { AnonymousUser, User, Admin, SuperAdmin }

impl Display for Role{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Role::AnonymousUser => String::from("AnonymousUser"),
            Role::User => String::from("User"),
            Role::Admin => String::from("Admin"),
            Role::SuperAdmin => String::from("SuperAdmin"),
        };
        write!(f, "{}", str)
    }
}