#[macro_use] extern crate failure;
#[macro_use] extern crate lazy_static;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate log;
extern crate simplelog;
extern crate serde;
extern crate serde_json;
extern crate scraper;
extern crate reqwest;
extern crate stopwords;
extern crate natural;

pub mod tmdb;
pub mod movie;
pub mod imdb;
mod porter;
