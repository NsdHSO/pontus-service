#[macro_use]
extern crate rocket;

use crate::pages::not_found::not_found;

mod pages;

#[get("/")]
fn world() -> &'static str {
    "Hello, world!"
}


#[launch]
fn rocket() -> _ {
    rocket::build().mount("/",routes![world]).register("/", catchers![not_found])
}
