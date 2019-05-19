use std::io;
use rocket::response::{NamedFile};
use rocket::http::RawStr;

#[get("/")]
pub fn index() -> io::Result<NamedFile> {
    NamedFile::open("static/index.html")
}

#[get("/search/<term>")]
pub fn response(term: &RawStr) -> String {
    format!("You typed in {}.", term)
}
