extern crate regex;
extern crate reqwest;
extern crate select;

use regex::Regex;
use select::document::Document;
use select::predicate::*;

const ROOT: &'static str = "https://en.wikipedia.org";
const SOURCE: &'static str = "https://en.wikipedia.org/wiki/Lists_of_films";

pub fn main() {
    
    let response = reqwest::get(SOURCE)
        .expect(&format!("Unable to read URL: {}", SOURCE));

    let doc = Document::from_read(response)
        .expect(&format!("Unable to parse HTML from URL: {}", SOURCE));

    let year_link_format = Regex::new(r"^/wiki/\d{4}_in_film$")
        .unwrap();

    let year_links = doc.find(Name("a"))
        .filter_map(|link| link.attr("href"))
        .filter(|link| year_link_format.is_match(link))
        .map(|link| ROOT.to_owned() + link)
        .collect::<Vec<_>>();

    let test = extract_year(&year_links[15]);

}

pub fn extract_year(url: &str) {

    let response = reqwest::get(url)
        .expect(&format!("Unable to read URL: {}", url));

    let doc = Document::from_read(response)
        .expect(&format!("Unable to parse HTML from URL: {}", url));

    let _ = doc.find(Name("a"))
        .filter(|link| link.attr("title").is_some())
        .for_each(|link| println!("{}", link.text()));
    
}
