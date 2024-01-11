use crate::db::connection::connection_init;
use crate::models::users::NewUser;
use crate::serials::users::{CreateUser, };
pub fn create_user(user: CreateUser) {
    println!("creating user {:?}", user);

    use crate::db::schema::users::dsl::*;

    let connection = connection_init();
    let new_user = NewUser {
        first_name: &user.first_name,
        last_name: &user.last_name,
        email: &user.email,
        phone_number: &user.phone_number,
        password: &user.password,
    };
}