use reqwest;
use failure::Error;
use std::str::FromStr;

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

macro_rules! metacritic_url {
    ( $id:expr ) => ( format!("http://www.imdb.com/title/{}/criticreviews?ref_=tt_ov_rt", $id) )
}

#[derive(Debug, Fail)]
pub enum IMDBError {
    #[fail(display = "{} is unavailable on IMDB", id)]
    Home { id: String },

    #[fail(display = "{} is missing link to poster", name)]
    Image { name: String },

    #[fail(display = "{} is missing rating", name)]
    Rating { name: String },

    #[fail(display = "{} is missing poster", name)]
    Poster { name: String },

    #[fail(display = "{} is missing IMDB score", name)]
    IMDBScore { name: String },

    #[fail(display = "{} is missing Metacritic score", name)]
    MetacriticScore { name: String },

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
    static ref RATING: Selector = Selector::parse("meta[itemprop=contentRating][content]").unwrap();

    static ref SCORE_VALUE: Selector = Selector::parse("span[itemprop=ratingValue]").unwrap();
    static ref SCORE_COUNT: Selector = Selector::parse("span[itemprop=ratingCount]").unwrap();

    static ref HYPERLINK: Regex = Regex::new(r"(\(\s*)?<a[^>]*>([^<]*)</a>(?:\s*\)\s*)?").unwrap();
    static ref WRITTEN_BY: Regex = Regex::new(r"(?s:\s*<em.*>\s*)").unwrap();
    static ref RESIZE: Regex = Regex::new(r"@\._V1_.*\.jpg").unwrap();
}

pub struct IMDB {
    id: String,
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

        Ok(IMDB { id: id.to_owned(), name: name.to_owned(), home })
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
                .next()
                .map(|element| element.value().attr("content").unwrap())
                .map(|link| RESIZE.replace_all(link, r"@._V1_.jpg").to_string())
                .ok_or(IMDBError::Image { name: self.name.clone() })?
                .to_owned()
        )
    }

    pub fn get_rating(&self) -> Result<String, Error> {
        Ok(
            self.home.select(&*RATING) 
                .map(|element| element.value().attr("content").unwrap())
                .next()
                .ok_or(IMDBError::Rating { name: self.name.clone() })?
                .to_owned()
        )
    }

    pub fn get_imdb_score(&self) -> Result<(f32, i32), Error> {
        let value = self.home.select(&*SCORE_VALUE) 
            .map(|element| f32::from_str(&element.inner_html()))
            .next()
            .ok_or(IMDBError::IMDBScore { name: self.name.clone() })??;
        
        let count = self.home.select(&*SCORE_COUNT) 
            .map(|element| i32::from_str(&element.inner_html().replace(",", "")))
            .next()
            .ok_or(IMDBError::IMDBScore { name: self.name.clone() })??;

        Ok((value, count))
    }

    pub fn get_metacritic_score(&self) -> Result<(f32, i32), Error> {

        let metacritic = Html::parse_document(
            &reqwest::get(&metacritic_url!(self.id))?.text()?
        );

        let value = metacritic.select(&*SCORE_VALUE)
            .map(|element| f32::from_str(&element.inner_html()))
            .next()
            .ok_or(IMDBError::MetacriticScore { name: self.name.clone() })??;

        let count = metacritic.select(&*SCORE_COUNT)
            .map(|element| i32::from_str(&element.inner_html().replace(",", "")))
            .next()
            .ok_or(IMDBError::MetacriticScore { name: self.name.clone() })??;

        Ok((value, count))
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
