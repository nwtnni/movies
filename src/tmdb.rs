use reqwest::get;
use serde_json::from_str;
use failure::Error;
use std::{thread, time};

#[derive(Deserialize)]
struct MovieID {
    pub id: i32
}

#[derive(Deserialize)]
pub struct Genre {
    pub name: String,
}

#[derive(Deserialize)]
pub struct Movie {
    pub id: i32,
    pub imdb_id: String,
    pub title: String,
    pub genres: Vec<Genre>,
    pub homepage: Option<String>, 
    pub original_language: Option<String>,
    pub popularity: Option<f32>,
    pub release_date: Option<String>,
    pub revenue: Option<f32>,
    pub runtime: Option<i32>, 
    pub tagline: Option<String>,
    pub vote_average: Option<f32>,
}

impl Movie {
    pub fn is_english(&self) -> bool {
        Some("en".to_owned()) == self.original_language
    }
}

#[derive(Deserialize)]
struct Keyword {
    pub name: String
}

#[derive(Deserialize)]
struct Keywords {
    pub keywords: Vec<Keyword>
}

#[derive(Deserialize)]
struct Page {
    pub total_pages: i32,
    pub results: Vec<MovieID>,
}

lazy_static! {
    static ref DELAY: time::Duration = time::Duration::from_millis(250);
}

pub fn get_page(key: &str, page: i32) -> Result<Vec<i32>, Error> {
    thread::sleep(*DELAY);

    let url = format!(
        "https://api.themoviedb.org/3/movie/popular?api_key={}&language=en-US&page={}",
        key,
        page
    );

    let page: Page = from_str(&get(&url)?.text()?)?;

    Ok(
        page.results
            .into_iter()
            .map(|movie| movie.id)
            .collect()
    )
}

pub fn get_movie(key: &str, id: i32) -> Result<Movie, Error> {
    thread::sleep(*DELAY); 

    let url = format!(
        "https://api.themoviedb.org/3/movie/{}?api_key={}&language=en-US",
        id,
        key
    );

    Ok(from_str(&get(&url)?.text()?)?)
}

pub fn get_keywords(key: &str, id: i32) -> Result<Vec<String>, Error> {
    thread::sleep(*DELAY); 

    let url = format!(
        "https://api.themoviedb.org/3/movie/{}/keywords?api_key={}&language=en-US",
        id,
        key
    );

    let keywords: Keywords = from_str(&get(&url)?.text()?)?;
    
    Ok(
        keywords.keywords
            .into_iter()
            .map(|word| word.name)
            .collect()
    )
}
