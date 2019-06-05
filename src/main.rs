#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
#[macro_use] extern crate serde_derive;
extern crate rocket_contrib;
extern crate regex;
extern crate reqwest;
extern crate select;
extern crate scraper;
extern crate rand;


mod routes;
use crate::routes::{ static_files, get, post };
use rocket_contrib::templates::Template;

fn rocket() -> rocket::Rocket {
    rocket::ignite().attach(Template::fairing()).mount("/",
    routes![static_files::file,get::index,get::response,post::process],)
}

fn main() {
    rocket().launch();
}
