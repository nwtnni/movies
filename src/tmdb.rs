use reqwest;
use serde_json;
use failure::Error;
use std::{thread, time};

#[derive(Deserialize)]
pub struct MovieID {
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
pub struct Keyword {
    pub name: String
}

#[derive(Deserialize)]
pub struct Keywords {
    pub keywords: Vec<Keyword>
}

#[derive(Deserialize)]
pub struct Page {
    pub total_pages: i32,
    pub results: Vec<MovieID>,
}

lazy_static! {
    static ref DELAY: time::Duration = time::Duration::from_millis(250);
}

pub fn get_page(key: &str, page: i32) -> Result<Page, Error> {
    thread::sleep(*DELAY);
    let url = format!("https://api.themoviedb.org/3/movie/popular?api_key={}&language=en-US&page={}", key, page);
    let data = reqwest::get(&url)?.text()?;
    Ok(serde_json::from_str(&data)?)
}

pub fn get_movie(key: &str, id: i32) -> Result<Movie, Error> {
    thread::sleep(*DELAY); 
    let url = format!("https://api.themoviedb.org/3/movie/{}?api_key={}&language=en-US", id, key);
    let data = reqwest::get(&url)?.text()?;
    Ok(serde_json::from_str(&data)?)
}

pub fn get_keywords(key: &str, id: i32) -> Result<Keywords, Error> {
    thread::sleep(*DELAY); 
    let url = format!("https://api.themoviedb.org/3/movie/{}/keywords?api_key={}&language=en-US", id, key);
    let data = reqwest::get(&url)?.text()?;
    Ok(serde_json::from_str(&data)?)
}
