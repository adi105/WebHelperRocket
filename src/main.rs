#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;

mod routes;
use crate::routes::{ static_files, get, post };

fn rocket() -> rocket::Rocket {
    rocket::ignite().mount("/",
    routes![static_files::file,get::index,get::response,post::process],)
}

fn main() {
    rocket().launch();
}
