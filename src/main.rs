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

lazy_static! {
    static ref KEY: String = env::var("TMDB_API_KEY")
        .expect("Could not find environment variable TMDB_API_KEY.");
}

pub fn from_tmdb(id: i32) -> Result<(tmdb::Movie, Vec<String>), Error> {
    Ok((tmdb::get_movie(&*KEY, id)?, tmdb::get_keywords(&*KEY, id)?))
}

pub fn main() {
    for n in 1..2 {
        if let Ok(page) = tmdb::get_page(&*KEY, n) {
            for id in page {
                if let Ok((m, _)) = from_tmdb(id) {
                    if let Ok(link) = imdb::test(&m.imdb_id) {
                        println!("Saving poster for {}...", m.title);
                        let mut file = File::create(format!("{}.jpg", m.title)).unwrap();
                        let mut poster = reqwest::get(&link).unwrap();
                        poster.copy_to(&mut file).unwrap();
                    } else {
                        println!("Could not find poster for {}.", m.title);
                    }
                }
            }
        }
    }
}
