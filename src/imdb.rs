use reqwest;
use failure::Error;

use scraper::Html;
use scraper::Selector;

/// Returns the website of movie with IMDB ID $id
macro_rules! home_url {
    ( $id:expr ) => ( format!("http://www.imdb.com/title/{}/", $id) )
}

/// Returns the absolute path of IMDB relative path $path
macro_rules! abs_url {
    ( $path:expr ) => ( format!("http://www.imdb.com{}", $path) )
}

#[derive(Debug, Fail)]
enum IMDBError {
    #[fail(display = "Missing link")]
    MissingLink
}

lazy_static! {
    static ref POSTER: Selector = Selector::parse(".poster a[href]").unwrap();
}

lazy_static! {
    static ref IMAGE: Selector = Selector::parse("meta[property=\"og:image\"][content]").unwrap();
}

/// Returns the URL of the poster of movie with IMDB ID [id]
pub fn get_poster(id: &str) -> Result<String, Error> {

    let url = home_url!(id);
    println!("{}", &url);
    let mut home = reqwest::get(&url)?;
    let link = Html::parse_document(&home.text()?)
        .select(&*POSTER)
        .map(|element| element.value().attr("href").unwrap())
        .next()
        .ok_or(IMDBError::MissingLink)?
        .to_owned();
    
    let mut poster = reqwest::get(&abs_url!(link))?;
    println!("{}", abs_url!(link));

    Ok(
        Html::parse_document(&poster.text()?)
            .select(&*IMAGE)
            .map(|element| element.value().attr("content").unwrap())
            .next()
            .ok_or(IMDBError::MissingLink)?
            .to_owned()
    )
}
