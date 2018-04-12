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

const PAGES: i32 = 999;

pub fn main() {

    let mut tmdb = TMDB::default();
    let _ = WriteLogger::init(LevelFilter::Info, Config::default(), File::create("movies.log").unwrap());
    let _ = create_dir("posters");
    let _ = create_dir("movies");
    let mut index = File::create("movies.json").unwrap();
    let _ = index.write(b"[\n");

    for n in 1..PAGES {
        match tmdb.get_page(n) {
        | Err(error) => error!("Page error: {}", error),
        | Ok(page) => {
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
        },
        }
    }

    let _ = index.write(b"\n]");
}
