use reqwest;
use failure::Error;

use regex::Regex;
use regex::Captures;
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
pub enum IMDBError {
    #[fail(display = "{} is unavailable on IMDB", id)]
    Home { id: String },

    #[fail(display = "{} is missing link to poster", name)]
    Image { name: String },

    #[fail(display = "{} is missing poster", name)]
    Poster { name: String },

    #[fail(display = "{} is missing summary", name)]
    Summary { name: String },

    #[fail(display = "{} is missing synopsis", name)]
    Synopsis { name: String },
}

lazy_static! {
    static ref POSTER: Selector = Selector::parse(".poster a[href]").unwrap();
    static ref IMAGE: Selector = Selector::parse("meta[property=\"og:image\"][content]").unwrap();
    static ref SUMMARY: Selector = Selector::parse("#titleStoryLine [itemprop=description] p").unwrap();
    static ref SYNOPSIS: Selector = Selector::parse("#titleStoryLine .see-more a[href]").unwrap();
    static ref TEXT: Selector = Selector::parse("#plot-synopsis-content .ipl-zebra-list__item").unwrap();

    static ref HYPERLINK: Regex = Regex::new(r"(\(\s*)?<a[^>]*>([^<]*)</a>(?:\s*\)\s*)?").unwrap();
    static ref WRITTEN_BY: Regex = Regex::new(r"(?s:\s*<em.*>\s*)").unwrap();
}

pub struct IMDB {
    name: String,
    home: Html,
}

impl IMDB {
    pub fn new(id: &str, name: &str) -> Result<Self, Error> {
        let home = Html::parse_document(
            &reqwest::get(&home_url!(id))
                .map_err(|_| IMDBError::Home { id: id.to_owned() })?
                .text()?
        );
        Ok(IMDB { name: name.to_owned(), home })
    }

    /// Returns the URL of the poster of movie with IMDB ID [id]
    pub fn get_poster(&self) -> Result<String, Error> {

        let link = self.home.select(&*POSTER)
            .map(|element| element.value().attr("href").unwrap())
            .next()
            .ok_or(IMDBError::Poster { name: self.name.clone() })?
            .to_owned();
        
        let poster = reqwest::get(&abs_url!(link))?.text()?;

        Ok(
            Html::parse_document(&poster)
                .select(&*IMAGE)
                .map(|element| element.value().attr("content").unwrap())
                .next()
                .ok_or(IMDBError::Image { name: self.name.clone() })?
                .to_owned()
        )
    }

    pub fn get_summary(&self) -> Result<String, Error> {
        Ok(
            self.home.select(&*SUMMARY)
                .map(|element| {
                    let s = HYPERLINK.replace_all(element.inner_html().trim(), |caps: &Captures| {
                        if let None = caps.get(1) { caps[2].to_owned() } else { "".to_owned() }
                    }).to_string();
                    WRITTEN_BY.replace_all(&s, "").to_string()
                })
                .filter(|summary| !summary.is_empty())
                .next()
                .ok_or(IMDBError::Summary { name: self.name.clone() })?
        )
    }

    pub fn get_synopsis(&self) -> Result<String, Error> {
        let link = self.home.select(&*SYNOPSIS)
            .map(|element| element.value().attr("href").unwrap())
            .next()
            .ok_or(IMDBError::Synopsis { name: self.name.clone() })?
            .to_owned();

        let synopsis = reqwest::get(&abs_url!(link))?.text()?;

        Ok(
            Html::parse_document(&synopsis)
                .select(&*TEXT)
                .filter(|element| element.value().id() != Some("no-synopsis-content"))
                .map(|element| {
                    element.text()
                        .map(|s| s.trim().to_owned())
                        .collect::<Vec<_>>()
                        .join("\n")
                })
                .filter(|synopsis| !synopsis.is_empty())
                .next()
                .ok_or(IMDBError::Synopsis { name: self.name.clone() })?
        )
    }
}
