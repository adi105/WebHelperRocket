// We can add special easter eggs to this file. Trying to have some fun.
use regex::Regex;
use scraper::{ Selector, Html };
use rand::Rng;

pub fn ping_pong() -> String {
    return "Pong".to_string()
}

#[test]
fn test_pingpong() {
    assert_eq!(ping_pong(), "Pong");
}

pub fn riddle_generator() -> Vec<String> {
    // url for a riddle generator
    let riddle_url: String = "https://examples.yourdictionary.com/examples-of-riddles.html".to_string();

    // Get the riddle webpage
    let mut req = reqwest::get(&*riddle_url).unwrap();
    assert!(req.status().is_success());

    // Extract the body.
    let body = req.text().unwrap();

    // Generate a random number between 1 and 17 to select a random riddle.
    let mut rng = rand::thread_rng();
    let mut retval: Vec<String> = Vec::new();
    // Parse the HTML and use CSS selectors
    let frag = Html::parse_document(&body);
    let css_selector = Selector::parse(format!("#article_intro > ul > li:nth-child({})", rng.gen_range(1, 18)).as_str()).unwrap();
    for j in frag.select(&css_selector) {
        let text = j.text().collect::<Vec<_>>();
        retval.push(format!("{}", text[0]));
    }

    // at this point, retval contains one of the 17 riddles present on the webpage we parsed.
    return retval;
}

// Function that mimics the echo command in unix
pub fn resp_echo(term: String) -> String {
    return term;
}

#[test]
fn test_echo() {
    assert_eq!(resp_echo("Choice".to_string()), "Choice");
    let arg: String = "Hello world!".to_string();
    assert_eq!(resp_echo(arg), "Hello world!");
}

pub fn get_ip() -> Vec<String> {
    // Website which obtains our IP Address
    let ip_url = "https://whatismyipaddress.com/".to_string();

    let mut req = reqwest::get(&*ip_url).unwrap();
    assert!(req.status().is_success());

    let body = req.text().unwrap();
    let mut retval: Vec<String> = Vec::new();

    let frag = Html::parse_document(&body);
    let css_selector = Selector::parse("#ipv4 > a").unwrap();
    for j in frag.select(&css_selector) {
        let text = j.text().collect::<Vec<_>>();
        retval.push(format!("{}", text[0]));
    }
    return retval;
}
