#[macro_use]
extern crate rocket;

use crate::pages::not_found::not_found;
use dotenvy::dotenv;
mod models;
mod pages;

#[get("/")]
fn world() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    dotenv().ok();
    rocket::build()
        .mount("/", routes![world])
        .register("/", catchers![not_found])
}
