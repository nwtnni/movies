#[macro_use]
extern crate lazy_static;
extern crate movies;
extern crate failure;

use std::env;
use failure::Error;
use movies::tmdb;

lazy_static! {
    static ref KEY: String = env::var("TMDB_API_KEY")
        .expect("Could not find environment variable TMDB_API_KEY.");
}

pub fn from_tmdb(id: i32) -> Result<(tmdb::Movie, Vec<String>), Error> {
    Ok((tmdb::get_movie(&*KEY, id)?, tmdb::get_keywords(&*KEY, id)?))
}

pub fn main() {
    for n in 1..100 {
        if let Ok(page) = tmdb::get_page(&*KEY, n) {
            for id in page {
                if let Ok((m, _)) = from_tmdb(id) {
                    println!("{}", m.title)        
                }
            }
        }
    }
}
