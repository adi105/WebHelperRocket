use crate::routes::{ calculator };
use std::io;
use rocket::response::{NamedFile};
use rocket::http::RawStr;
use rocket::request::{Form, FromFormValue};
use rocket::response::Redirect;
use url::form_urlencoded;
use rocket_contrib::templates::{Template, handlebars};
use regex::Regex;

use handlebars::{Helper, Handlebars, Context, RenderContext, Output, HelperResult, JsonRender};



#[derive(Serialize)]
pub struct TemplateContext {
    query: String,
    items: Vec<String>,
    parent: &'static str,
}

#[derive(FromForm)]
pub struct Request {
    searchterm: String,
    //we can add more if we want later on, for other form options...
}

// This is the function which responds to the input. We use handlebars to create dynamic HTML templates
// which make it simple to display our data on the webpage.
#[post("/search", data = "<data>")]
pub fn process(data: Form<Request>) -> Template {
    // Create a regex to detect calculator syntax
    let calc_reg = Regex::new(r"[c,C]alculate*").unwrap();
    let qry = &data.searchterm;
    if calc_reg.is_match(qry) {
        // this means we have a calculator query.
        let expr: Vec<&str> = calc_reg.split(qry).collect();
        let result = calculator::calculate(expr[1].to_string()).unwrap();
        // Now create the template
        return Template::render("result", &TemplateContext {
            query: qry.to_string(),
            items: vec![result],
            parent: "layout",
        });
    }

    // A placeholder template for when we get an invalid query.
    Template::render("result", &TemplateContext {
        query: "null".to_string(),
        items: vec!["0".to_string(), "0".to_string(), "0".to_string()],
        parent: "layout",
    })
}
