use reqwest;
use failure::Error;

use select::document::Document;
use select::predicate::{Predicate, Attr, Class, Name};

macro_rules! home_url {
    ( $id:expr ) => ( format!("http://www.imdb.com/title/{}/", $id) )
}

macro_rules! abs_url {
    ( $path:expr ) => ( format!("http://www.imdb.com{}", $path) )
}

#[derive(Debug, Fail)]
enum IMDBError {
    #[fail(display = "Missing link")]
    MissingLink
}

pub fn test(id: &str) -> Result<String, Error> {

    let url = home_url!(id);
    let home = reqwest::get(&url)?;
    let link = Document::from_read(home)?
        .find(Class("poster").child(Name("a")))
        .filter_map(|node| node.attr("href"))
        .map(|rel| abs_url!(rel))
        .next()
        .ok_or(IMDBError::MissingLink)?;
    
    let poster = reqwest::get(&link)?;
    println!("Found poster site {}", link);

    Ok(
        Document::from_read(poster)?
            .find(Name("meta").and(Attr("property", "og:image")))
            .filter_map(|node| node.attr("content"))
            .map(|link| link.to_owned())
            .next()
            .ok_or(IMDBError::MissingLink)?
    )
}
