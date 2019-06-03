//=======================================================================
// Please note, this web scraper takes advantage of static html elements I noticed within the dictionary's
// source. I noticed that the definitions are all marked with the <span class="ind">(payload)</span>,
// and as such, I decided to use regex to strip the payload out of these. I also manually break after the first
// definition is read. As such, this is pretty unstable and if the webpage design changes it won't work.
// Works like a charm at this moment, though! :)
//    -Adrian
//========================================================================

use scraper::{ Html };
use regex::Regex;

// this function serves to obtain the definition of the word provided from dictionary.com
// Since I was not able to find an API for definitions native to Rust, I use a web scraping approach.
pub fn get_definition(word: String) -> Vec<String> {
    // Creation of static url that we will use
    let mut dict_url: String = "https://en.oxforddictionaries.com/definition/".to_string();
    // The Vector that will hold our result output. whether it is a def or an error.
    let mut retvec: Vec<String> = Vec::new();
    // Sanitize some of the input. We want single words, no spaces, etc
    if word.contains(" ") {
        retvec.push("Error: Space Detected".to_string());
        return retvec;
    }

    //make sure all characters are alphanum
    for c in word.chars() {
        if c.is_alphanumeric() == false {
            retvec.push("Error: non-alphanum character detected.".to_string());
            return retvec;
        }
    }

    //SANITIZATION DONE============================================================================
    dict_url.push_str(&word);
    //move to a non mutable, usable variable once we are done changing things.
    let final_url = &*dict_url;

    //use created url to create a GET request
    let mut req = reqwest::get(final_url).unwrap();
    assert!(req.status().is_success()); //this assert is standard when using reqwest crate

    //process the body of the html
    let body = req.text().unwrap();

    // we are going to need to use regex. A separate function will handle this.
    let result: String = process_string(body);
    retvec.push(result);
    return retvec;
}

// This function serves to perform processing using regex, which will find the definitions of the given word.\
// It assumes that the webpage exists beforehand, so we need some error checking before calling it.
pub fn process_string(input: String) -> String {

    let re: Regex = Regex::new("<span class=\"ind\">(.*?)</span>").unwrap();
    //the above regex string will match for the class which is before the definitions of the word.
    // We now need to get the first definition. The following loop is capable of getting all definitions,
    // but I set a break to only get the first one.
    let mut retval: String = "".to_string();
    for cap in re.captures_iter(&input) {
        let target = &cap[1];
        //println!("{}", target);
        retval = target.to_string();
        break;
    }
    // at this point, retval[0] will either store our data, or an empty string. This is as good to me as
    // creating a Option because I can use a binary comparison regardless.
    return retval;
}

//UNIT TESTS=======================================================================================
#[test]
fn test_regex() {
    // testing general functionality in a simple, one definition use case
    assert_eq!(process_string("<span class=\"ind\">A person who habitually seeks to harm or intimidate those whom they perceive as vulnerable.</span>".to_string()),
                                "A person who habitually seeks to harm or intimidate those whom they perceive as vulnerable.");
    // making sure only the first definition is grabbed from a mutli-definition page
    assert_eq!(process_string("<span class=\"ind\">A nice old lady</span> <span class=\"ind\">A mean old man.</span>".to_string()),
                                "A nice old lady");
}
