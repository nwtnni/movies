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

pub fn from_tmdb(id: i32) -> Result<(tmdb::Movie, tmdb::Keywords), Error> {
    Ok((tmdb::get_movie(&*KEY, id)?, tmdb::get_keywords(&*KEY, id)?))
}

pub fn main() {
    for page in 1..100 {
        if let Ok(p) = tmdb::get_page(&*KEY, page) {
            for movie in p.results {
                if let Ok((m, _)) = from_tmdb(movie.id) {
                    println!("{}", m.title)        
                }
            }
        }
    }
}
