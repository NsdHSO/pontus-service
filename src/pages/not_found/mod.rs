use rocket::http::Status;
use std::fs;
use rocket::response::content::RawHtml;

#[catch(404)]
pub fn not_found() -> Result<RawHtml<String>, Status> {
    let file_path = "src/templates/not_found.html";

    match fs::read_to_string(file_path) {
        Err(why) => panic!("couldn't read {}: {}", file_path, why),
        Ok(content) => Ok(RawHtml(content)),
    }
}
