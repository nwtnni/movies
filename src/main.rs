#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

extern crate regex;
extern crate reqwest;

use std::{thread, time, env};

#[derive(Deserialize)]
pub struct Movie {
    id: i32,
    vote_count: i32,
    vote_average: f32,
    title: String,
    popularity: f32,
    overview: String,
    release_date: String,
}

#[derive(Deserialize)]
pub struct Response {
    total_pages: i32,
    results: Vec<Movie>,
}

pub fn scrape(page: i32) -> Response {

    let url = format!("https://api.themoviedb.org/3/movie/popular?api_key={}&language=en-US&page={}", env::var("TMDB_API_KEY").unwrap(), page);
    let data = reqwest::get(&url).unwrap().text().unwrap();
    serde_json::from_str(&data).unwrap()

}

pub fn main() {
    
    let delay = time::Duration::from_millis(250);
    let first = scrape(1);
    let total = first.total_pages;
    println!("{}", total);

    for page in 1..total {
        thread::sleep(delay);
        for movie in scrape(page).results {
            println!("{}: {}, {}", movie.id, movie.title, movie.vote_average);
        }
    }
}
