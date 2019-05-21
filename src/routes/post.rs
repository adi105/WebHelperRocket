use std::io;
use rocket::response::{NamedFile};
use rocket::http::RawStr;
use rocket::request::{Form, FromFormValue};
use rocket::response::Redirect;
use url::form_urlencoded;

//#[derive(Debug)]
//struct DataEntry<'r>(&'r str);

#[derive(FromForm)]
pub struct Request {
    searchterm: String,
    //we can add more if we want later on, for other form options...
}

//some bounds checking, this is where we can "enable" new features
/*
impl<'v> FromFormValue<'v> for DataEntry<'v> {
    type Error = &'static str;

    fn from_form_value(v: &'v RawStr) -> Result<Self, Self::Error> {
        //this is where we would place limitations on the string
        if v.len() < 1 {
            Err("This is too short!")
        } else {
            Ok(DataEntry(v.as_str()))
        }
    }
}
*/

#[post("/search", data = "<data>")]
pub fn process(data: Form<Request>) -> Result<Redirect, String> {
    if data.searchterm == "Hello!" {
        Ok(Redirect::to("/search/Hello"))
    } else {
        Err(format!("Unknown search term, '{}'.", data.searchterm))
    }
}
