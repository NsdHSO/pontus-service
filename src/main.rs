#[macro_use]
extern crate rocket;

use rocket::tokio::time::{sleep, Duration};

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/iancu")]
fn index_iancu() -> &'static str {
    "Hello, Iancu!"
}

#[get("/delay/<seconds>")]
async fn delay_on(seconds: u64) -> String {
    sleep(Duration::from_secs(seconds)).await;
    format!("{} seconds", seconds)
}

#[catch(404)]
fn not_found() -> &'static str {
    "not found!"
}
#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/iancu", routes![index_iancu])
        .mount("/", routes![delay_on])
        .mount("/", routes![index])
        .register("/", catchers![not_found])
}
