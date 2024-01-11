use std::fmt::{Debug,};
use diesel::deserialize::{self,};

use serde::{Serialize, Deserialize};
use diesel::prelude::*;


use crate::models::roles::Role;


// #[allow(unused)]
// #[derive(Debug, Serialize, Deserialize, Clone)]
#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::db::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User{
    id: i32,
    first_name: String,
    last_name: String,
    email: String,
    phone_number: String,
    password: String,
    role: String,
    date_of_birth: Option<String>,
    active: bool,
    created_at: String,
    updated_at: Option<String>,
}

use crate::db::schema::users;
#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUser<'a> {
    pub first_name: &'a str,
    pub last_name: &'a str,
    pub email: &'a str,
    pub phone_number: &'a str,
    pub password: &'a str,
}


// impl User{
//     pub fn new(first_name: String, last_name: String, email: String, phone_number: String, dob: Option<NaiveDate>, password: String) -> Self{
//         Self{
//             id: Uuid::new_v4(),
//             first_name,
//             last_name,
//             email,
//             password,
//             phone_number,
//             date_of_birth: dob,
//             active: false,
//             role: Role::AnonymousUser,
//             created_at: chrono::Utc::now().naive_utc(),
//             updated_at: None,
//         }
//     }
// }

impl User{

}