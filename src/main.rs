mod models;
mod api;
mod db;
mod serials;

#[macro_use] extern crate rocket;

use models::{users::User,roles::Role};
use serde::{ Serialize, Deserialize};
use std::sync::{ Mutex, Arc };

#[get("/live")]
fn api_live() -> &'static str {
    "It works"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![api_live])
}