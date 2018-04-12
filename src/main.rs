#[macro_use] extern crate log;
extern crate simplelog;
extern crate movies;
extern crate failure;
extern crate reqwest;
extern crate serde_json;

use std::io::Write;
use std::fs::File;
use std::fs::create_dir;
use simplelog::*;

use movies::tmdb::*;
use movies::movie::Movie;

pub fn main() {

    let mut tmdb = TMDB::default();
    let _ = WriteLogger::init(LevelFilter::Info, Config::default(), File::create("movies.log").unwrap());
    let _ = create_dir("posters").unwrap();
    let _ = create_dir("movies").unwrap();
    let mut index = File::create("movies.json").unwrap();
    index.write(b"[\n").unwrap();

    for n in 1..2 {
        if let Ok(page) = tmdb.get_page(n) {
            for id in page {
                match Movie::save(id, &mut tmdb) {
                | Err(warning) => warn!("{}", warning),
                | Ok(movie) => {
                    info!("Succesfully processed {}", movie.title);

                    if let Ok(json) = serde_json::to_string(&movie) {
                        if let Err(e) = index.write_all(format!("    {},\n", json).as_bytes()) {
                            error!("Write error: {}", e);
                        }
                    }
                }
                }
            }
        }
    }

    index.write(b"\n]").unwrap();
}
