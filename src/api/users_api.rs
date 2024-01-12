use rocket::{post, get, put, delete, State};
use rocket::serde::json::Json;
use crate::api::helpers::user_api_helper::{create_user_helper, delete_user_helper, get_user_helper, get_users_helper, update_user_helper};
use crate::models::users::{User};
use crate::serials::users::{
    CreateUser,
    ErrUser,
    UpdateUser,
};

// create user api
#[post("/users", format = "json", data = "<user>")]
pub async fn create_user(user: Json<CreateUser>) -> Json<Result<User, ErrUser>> {
    let user_struct = CreateUser {
        first_name: user.first_name.clone(),
        last_name: user.last_name.clone(),
        email: user.email.clone(),
        phone_number: user.phone_number.clone(),
        password: user.password.clone(),
    };

    let new_user = create_user_helper(user_struct);
    if let Some(user) = new_user {
        return Json(Ok(user));
    }
    let error_message = ErrUser { status_code: 500, message: "Something went wrong when creating user".to_string() };
    return Json(Err(error_message));
}

// get user api
#[get("/users/<user_id>")]
pub async fn get_user(user_id: i32) -> Json<Result<User, ErrUser>> {
    let user = get_user_helper(user_id);
    if let Some(found_user) = user {
        return Json(Ok(found_user));
    }
    let error_message: ErrUser = ErrUser { status_code: 404, message: "user not found".to_string() };
    return Json(Err(error_message));
}

// get users api
#[get("/users")]
pub async fn get_users() -> Json<Vec<User>> {
    let users = get_users_helper();
    Json(users)
}

// update user api
#[put("/users/<user_id>", format = "json", data = "<user>")]
pub async fn update_user(user_id: i32, user: Json<UpdateUser>) -> Json<Result<User, ErrUser>> {
    let response = update_user_helper(&user);

    if let Some(updated_user) = response {
        return Json(Ok(updated_user));
    }

    let error_message = ErrUser { status_code: 500, message: "soemthing went wrong while updating user".to_string() };
    return Json(Err(error_message));
}

// delete user api
#[delete("/users/<user_id>")]
pub async fn delete_user(user_id: i32) -> Json<Result<String, String>> {
    let response = delete_user_helper(user_id);
    if let Some(result) = response {
        return Json(Ok("deleted successfully".to_string()));
    }
    return Json(Err("something went wrong while deleting".to_string()));
}

