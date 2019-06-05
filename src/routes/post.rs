//=============================================================
use crate::routes::{ calculator, dict_fetch, cl_sweeper, easter };
use rocket::request::{ Form };
use rocket::response::Redirect;
use rocket_contrib::templates::{Template};
use regex::Regex;
//==============================================================

// This is a struct which gives information to the template. We need to fill in the "blanks" and tell
// handlebars what we want "query", "items", and the "parent" to be for each handlebars page.
#[derive(Serialize)]
pub struct TemplateContext {
    query: String,
    items: Vec<String>,
    parent: &'static str,
}

// This is to catch the searchterm from the form. Part of Rocket's form syntax
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
    let calc_reg = Regex::new(r"[cC]alculate +(.*) *").unwrap();
    // Create a regex to detect definition syntax
    let def_reg = Regex::new(r"[dD]efine +(.*) *").unwrap();
    // Create a regex to detect craigslist syntax
    let cl_reg = Regex::new(r"[cC]raigslist +(.*) *").unwrap();
    // Create regex to catch riddle
    let riddle_reg = Regex::new(r"[rR]iddle *").unwrap();
    // Regex for ping pong
    let ping_reg = Regex::new(r"[pP]ing *").unwrap();
    // Regex for echo
    let echo_reg = Regex::new(r"[eE]cho +(.*) *").unwrap();


    let qry = &data.searchterm;
    if riddle_reg.is_match(qry) {
        // This means that the user wants a riddle.
        let func_result = easter::riddle_generator();
        return Template::render("result", &TemplateContext {
            query: qry.to_string(),
            items: func_result,
            parent: "layout",
        });
    } else if ping_reg.is_match(qry) {
        let func_result = easter::ping_pong();
        return Template::render("result", &TemplateContext {
            query: qry.to_string(),
            items: vec![func_result],
            parent: "layout",
        });
    } else if echo_reg.is_match(qry) {
        let caps = echo_reg.captures(qry).unwrap();
        let func_result = easter::resp_echo(caps.get(1).unwrap().as_str().to_string());
        return Template::render("result", &TemplateContext {
            query: qry.to_string(),
            items: vec![func_result],
            parent: "layout",
        });
    } else if calc_reg.is_match(qry) {
        // this means we have a calculator query.
        let caps = calc_reg.captures(qry).unwrap();
        let func_result = calculator::calculate(caps.get(1).unwrap().as_str().to_string());
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
        let expr = def_reg.captures(qry).unwrap();
        let result = dict_fetch::get_definition(expr.get(1).unwrap().as_str().to_string());

        //template creation
        return Template::render("result", &TemplateContext {
            query: qry.to_string(),
            items: result,
            parent: "layout",
        });
    } else if cl_reg.is_match(qry) {
        let expr = cl_reg.captures(qry).unwrap();
        let result = cl_sweeper::cl_scraper(expr.get(1).unwrap().as_str().to_string());

        //template creation
        return Template::render("result", &TemplateContext {
            query: qry.to_string(),
            items: result,
            parent: "layout",
        });
    }

    // A placeholder template for when we get an invalid query.
    Template::render("result", &TemplateContext {
        query: "invalid".to_string(),
        items: vec!["Please reference available commands.".to_string()],
        parent: "layout",
    })
}
