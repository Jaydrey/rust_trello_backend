use std::fmt::{Debug,};
use diesel::deserialize::{self,};
use serde::{Serialize, Deserialize};
use diesel::prelude::*;



#[derive(Debug, Serialize, Deserialize, Queryable, Selectable, AsChangeset)]
#[diesel(table_name = crate::db::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User{
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub phone_number: String,
    pub password: String,
    pub role: String,
    pub date_of_birth: Option<String>,
    pub active: bool,
    pub created_at: String,
    pub updated_at: Option<String>,
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
    pub role: &'a str,
    pub active: &'a bool,
    pub created_at: &'a str,
}

impl User{

}