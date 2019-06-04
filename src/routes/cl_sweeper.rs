use scraper::{Selector, Html};

pub fn cl_scraper(url: String) -> Vec<String> {
  //creation of the URL=====================================================================
  let mut cl_url: String = "https://portland.craigslist.org/search/sss?query=".to_string();
  let mut new_url: String = "".to_string();

  for i in url.chars() {
    if i == ' ' {
      new_url.push('+');
    }
    else {
      new_url.push(i);
    }
  }

  cl_url.push_str(&new_url);
  cl_url.push_str("&sort=rel");

  let final_url = &*cl_url;

  // Scraping from the URL==================================================================
  let mut resp = reqwest::get(final_url).unwrap();
  assert!(resp.status().is_success());

  let mut query_to_parse: Vec<String> = Vec::new();
  let body = resp.text().unwrap();
  let fragment = Html::parse_document(&body);
  for i in 1..10 {
    query_to_parse.push(format!("#sortable-results > ul > li:nth-child({}) > p > a", i));
    //println!("{}",query_to_parse[i - 1]);
  }

  let mut retval: Vec<String> = Vec::new();
  for x in 0..9 {
    let selector = Selector::parse(&query_to_parse[x]).unwrap();
    for j in fragment.select(&selector) {
      let txt = j.text().collect::<Vec<_>>();
      retval.push(format!("{}", txt[0]));
      //println!("{}", txt[0]);
    }
  }

  println!("Printing the contents of the vector which holds first 10 CL results...");
  for j in 0..9 {
    println!("{}", retval[j]);
  }
  return retval
}
