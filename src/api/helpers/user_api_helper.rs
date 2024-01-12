use chrono::Utc;
use diesel::{QueryDsl, RunQueryDsl, SelectableHelper};
use crate::db::connection::connection_init;
use crate::models::roles::Role;
use crate::models::users::{NewUser, User};
use crate::serials::users::{CreateUser, UpdateUser};

pub fn create_user_helper(user: CreateUser) -> Option<User> {
    println!("creating user {:?}", user);

    use crate::db::schema::users::dsl::*;

    let mut connection = connection_init();
    let new_user = NewUser {
        first_name: &user.first_name,
        last_name: &user.last_name,
        email: &user.email,
        phone_number: &user.phone_number,
        password: &user.password,
        active: &false,
        role: &Role::User.to_string(),
        created_at: &Utc::now().to_rfc3339(),
    };

    let user = diesel::insert_into(users)
        .values(&new_user)
        .get_result::<User>(&mut connection).ok();
    return user;
}

pub fn get_users_helper() -> Vec<User>{
    println!("get users");

    use crate::db::schema::users::dsl::*;

    let mut connection = connection_init();

    let all_users = users.load(&mut connection).unwrap_or(vec![]);
    return all_users;
}

pub fn get_user_helper(user_id: i32) -> Option<User> {
    println!("get users");

    use crate::db::schema::users::dsl::*;

    let mut connection = connection_init();

    let user = users.find(user_id).select(User::as_select()).first(&mut connection).ok();
    return user;
}

pub fn update_user_helper(user: &UpdateUser) -> Option<User>{
    println!("update user {:?}", user);

    use crate::db::schema::users::dsl::*;

    let mut connection = connection_init();

    let user = User {
        id: user.id.clone(),
        first_name: user.first_name.clone(),
        last_name: user.last_name.clone(),
        email: user.email.clone(),
        phone_number: user.phone_number.clone(),
        password: user.password.clone(),
        role: user.role.clone(),
        date_of_birth: user.date_of_birth.clone(),
        active: user.active.clone(),
        created_at: user.created_at.clone(),
        updated_at: Some(Utc::now().to_rfc3339()),
    };

    let updated_user = diesel::update(users.find(user.id))
        .set(&user)
        .get_result::<User>(&mut connection)
        .ok();
    return updated_user;
}

pub fn delete_user_helper(user_id: i32) -> Option<usize>{
    println!("delete user by id {:?}", user_id);

    use crate::db::schema::users::dsl::*;

    let mut connection = connection_init();

    let response = diesel::delete(users.find(user_id))
        .execute(&mut connection)
        .ok();
    return response;
}