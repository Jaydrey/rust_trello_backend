use clap::{
    Args,
    Parser,
};
use serde::{Serialize, Deserialize};
#[derive(Debug, Args, Serialize, Deserialize)]
pub struct CreateUser {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub phone_number: String,
    pub password: String,
}

#[derive(Debug, Args, Serialize, Deserialize)]
pub struct UpdateUser {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub phone_number: String,
    pub password: String,
    pub role: String,
    pub date_of_birth: Option<String>,
    pub created_at: String,
    pub active: bool,
}

#[derive(Debug, Args)]
pub struct DeleteUser {
    pub id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrUser {
    pub status_code: i16,
    pub message: String,
}
