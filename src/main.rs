#[macro_use]
extern crate rocket;

use crate::pages::not_found::not_found;

mod pages;

#[launch]
fn rocket() -> _ {
    rocket::build().register("/", catchers![not_found])
}
