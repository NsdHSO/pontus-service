use rocket::serde::json::Json;
use crate::models::response::NetworkResponse;

pub struct User {
    pub username: String,
    pub email: String,
    pub password: String,
    pub id: i32
}


