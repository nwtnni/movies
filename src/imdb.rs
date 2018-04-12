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
    Image,

    #[fail(display = "Missing poster")]
    Poster,

    #[fail(display = "Missing summary")]
    Summary,
}

lazy_static! {
    static ref POSTER: Selector = Selector::parse(".poster a[href]").unwrap();
}

lazy_static! {
    static ref IMAGE: Selector = Selector::parse("meta[property=\"og:image\"][content]").unwrap();
}

lazy_static! {
    static ref SUMMARY: Selector = Selector::parse("#titleStoryLine [itemprop=description] p").unwrap();
}

pub struct IMDB {
    home: Html,
}

impl IMDB {
    pub fn new(id: &str) -> Result<Self, Error> {
        let home = Html::parse_document(
            &reqwest::get(&home_url!(id))?.text()?
        );
        Ok(IMDB { home })
    }

    /// Returns the URL of the poster of movie with IMDB ID [id]
    pub fn get_poster(&self) -> Result<String, Error> {

        let link = self.home.select(&*POSTER)
            .map(|element| element.value().attr("href").unwrap())
            .next()
            .ok_or(IMDBError::Poster)?
            .to_owned();
        
        let poster = reqwest::get(&abs_url!(link))?.text()?;

        Ok(
            Html::parse_document(&poster)
                .select(&*IMAGE)
                .map(|element| element.value().attr("content").unwrap())
                .next()
                .ok_or(IMDBError::Image)?
                .to_owned()
        )
    }

    pub fn get_summary(&self) -> Result<String, Error> {
        Ok(
            self.home.select(&*SUMMARY)
                .map(|element| {
                    element.text()
                        .next()
                        .unwrap_or("")
                        .to_owned()
                })
                .next()
                .ok_or(IMDBError::Summary)?
        )
    }
}
