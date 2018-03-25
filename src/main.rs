#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

extern crate regex;
extern crate reqwest;

use std::{fs, thread, time, env};
use std::io::Write;

#[derive(Deserialize)]
pub struct MovieID { id: i32 }

#[derive(Deserialize)]
pub struct Genre {
    id: i32,
    name: String,
}

#[derive(Deserialize)]
pub struct MovieResponse {
    id: Option<i32>,
    genres: Vec<Genre>,
    homepage: Option<String>, 
    imdb_id: Option<String>,
    overview: Option<String>,
    original_language: Option<String>,
    popularity: Option<f32>,
    release_date: Option<String>, 
    revenue: Option<f32>,
    runtime: Option<i32>, 
    tagline: Option<String>,
    title: Option<String>,
    vote_average: Option<f32>,
}

impl MovieResponse {
    pub fn is_english(&self) -> bool {
        Some("en".to_owned()) == self.original_language
    }
}

#[derive(Serialize)]
pub struct Movie {
    id: Option<i32>,
    genres: Vec<String>,
    homepage: Option<String>, 
    imdb_id: Option<String>,
    overview: Option<String>,
    popularity: Option<f32>,
    release_date: Option<String>, 
    revenue: Option<f32>,
    runtime: Option<i32>, 
    tagline: Option<String>,
    title: Option<String>,
    vote_average: Option<f32>,
    keywords: Vec<String>, 
}

#[derive(Deserialize)]
pub struct Keyword { name: String }

#[derive(Deserialize)]
pub struct Keywords { keywords: Vec<Keyword> }

#[derive(Deserialize)]
pub struct Page {
    total_pages: i32,
    results: Vec<MovieID>,
}

pub fn get_page(delay: time::Duration, key: &str, page: i32) -> Page {
    thread::sleep(delay);
    let url = format!("https://api.themoviedb.org/3/movie/popular?api_key={}&language=en-US&page={}", key, page);
    let data = reqwest::get(&url).unwrap().text().unwrap();
    serde_json::from_str(&data).unwrap()
}

pub fn get_movie(delay: time::Duration, key: &str, id: i32) -> MovieResponse {
    thread::sleep(delay); 
    let url = format!("https://api.themoviedb.org/3/movie/{}?api_key={}&language=en-US", id, key);
    let data = reqwest::get(&url).unwrap().text().unwrap();
    serde_json::from_str(&data).unwrap()
}

pub fn get_keywords(delay: time::Duration, key: &str, id: i32) -> Keywords {
    thread::sleep(delay); 
    let url = format!("https://api.themoviedb.org/3/movie/{}/keywords?api_key={}&language=en-US", id, key);
    let data = reqwest::get(&url).unwrap().text().unwrap();
    serde_json::from_str(&data).unwrap()
}

pub fn create(m: MovieResponse, k: Keywords) -> Movie {
    Movie {
        id: m.id,
        genres: m.genres.into_iter().map(|genre| genre.name).collect(),
        homepage: m.homepage, 
        imdb_id: m.imdb_id,
        overview: m.overview,
        popularity: m.popularity,
        release_date: m.release_date, 
        revenue: m.revenue,
        runtime: m.runtime, 
        tagline: m.tagline,
        title: m.title,
        vote_average: m.vote_average,
        keywords: k.keywords.into_iter().map(|k| k.name).collect(),
    }
}

pub fn main() {
    
    let delay = time::Duration::from_millis(250);
    let key = env::var("TMDB_API_KEY").unwrap();
    let mut outfile = fs::File::create("movies.json").unwrap();

    outfile.write_all(b"[").unwrap();

    for page in 1..100 {
        for movie in get_page(delay, &key, page).results {
            
            let m = get_movie(delay, &key, movie.id);

            if !m.is_english() { continue }

            let k = get_keywords(delay, &key, movie.id);
            
            let result = create(m, k);

            outfile.write_all(serde_json::to_string(&result).unwrap().as_bytes()).unwrap();
            outfile.write_all(b",\n").unwrap();

        }
    }

    outfile.write_all(b"]").unwrap();
}
