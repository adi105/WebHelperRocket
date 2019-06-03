#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate regex;
#[macro_use] extern crate reqwest;
#[macro_use] extern crate select;
#[macro_use] extern crate scraper;


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
