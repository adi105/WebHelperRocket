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
    //experimentation with functions within this file below
    if data.searchterm == "3+5" {
        let mut result = calculator(data.searchterm);
        result.to_string();
        format!("Your result is '{}'.", result);
    }
    //=====================================================
    if data.searchterm == "Hello!" {
        Ok(Redirect::to("/search/Hello"))
    } else {
        Err(format!("Unknown search term, '{}'.", data.searchterm))
    }
}

/*
pub fn calculator(input: String) -> u64 {
    let first_letter = input.chars().next().unwrap();
    let x:u64 = first_letter.to_digit(10).unwrap().into();
    let symbol = input.chars().nth(1).unwrap();
    let y:u64 = input.chars().nth(2).unwrap().to_digit(10).unwrap().into();
    let mut retval:u64 = match symbol {
        '+' => x + y,
        '-' => x - y,
        '*' => 0,
    };
    return retval
}
*/
