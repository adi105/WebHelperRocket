use crate::routes::{ calculator, dict_fetch };
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
    // Create a regex to detect definition syntax
    let def_reg = Regex::new(r"[d,D]efine *").unwrap();

    let qry = &data.searchterm;
    if calc_reg.is_match(qry) {
        // this means we have a calculator query.
        let expr: Vec<&str> = calc_reg.split(qry).collect();
        let func_result = calculator::calculate(expr[1].to_string());
        let result: String;
        match func_result {
            Ok(n) => result = format!("{}", n),
            Err(e) => result = format!("{}", e)
        }
        // Now create the template
        return Template::render("result", &TemplateContext {
            query: qry.to_string(),
            items: vec![result],
            parent: "layout",
        });
    } else if def_reg.is_match(qry) {
        // This means we have a definition query.
        let expr: Vec<&str> = def_reg.split(qry).collect();
        let result = dict_fetch::get_definition(expr[1].to_string());

        //template creation
        return Template::render("result", &TemplateContext {
            query: qry.to_string(),
            items: result,
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
