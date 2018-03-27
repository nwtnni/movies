#[macro_use]
extern crate lazy_static;
extern crate movies;
extern crate failure;
extern crate reqwest;

use std::env;
use std::fs::File;
use failure::Error;
use movies::tmdb;
use movies::imdb;

pub fn main() {

    let mut tmdb = tmdb::TMDB::default();

    for n in 1..2 {
        if let Ok(page) = tmdb.get_page(n) {
            for id in page {
                if let Ok(movie) = tmdb.get_movie(id) {
                    if let Ok(link) = imdb::get_poster(&movie.imdb_id) {
                        println!("Saving poster for {}...", movie.title);
                        let mut file = File::create(format!("{}.jpg", movie.title)).unwrap();
                        let mut poster = reqwest::get(&link).unwrap();
                        poster.copy_to(&mut file).unwrap();
                    } else {
                        println!("Could not find poster for {}.", movie.title);
                    }
                }
            }
        }
    }
}
