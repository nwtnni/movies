use natural::tokenize::tokenize;
use std::collections::HashSet;
use stopwords::{NLTK, Language, Stopwords};
use failure::Error;

use std::io::Write;
use std::fs::File;
use serde_json;

use tmdb::*;
use imdb::*;
use porter::*;
use reqwest;

lazy_static! {
    static ref STOP_WORDS: HashSet<&'static str> = NLTK::stopwords(Language::English)
        .unwrap()
        .into_iter()
        .map(|&s| s)
        .collect();
}

#[derive(Deserialize, Serialize)]
pub struct Cast {
    pub character: String,
    pub name: String,  
}

#[derive(Deserialize, Serialize)]
pub struct Crew {
    pub job: String,
    pub name: String,
}

#[derive(Serialize)]
pub struct Movie {
    pub id: String,
    pub cast: Vec<Cast>,
    pub crew: Vec<Crew>,
    pub title: String,
    pub genres: Vec<String>,
    pub keywords: Vec<String>,
    pub popularity: f32,
    pub release_date: String,
    pub revenue: f32,
    pub runtime: i32, 
    pub summary: String,
    pub tokens: Vec<String>,
    pub vote_average: f32,
}

#[derive(Serialize)]
pub struct Index {
    pub id: String,
    pub title: String,
}

impl Movie {
    
    pub fn save(id: i32, tmdb: &mut TMDB) -> Result<Index, Error> {
        let movie = tmdb.get_raw_movie(id)?;
        let (cast, crew) = tmdb.get_people(id)?;
        let keywords = tmdb.get_keywords(id)?;

        let imdb = IMDB::new(&movie.imdb_id, &movie.title)?;
        let summary = imdb.get_summary()?;
        let synopsis = imdb.get_synopsis()?.to_lowercase();
        let tokens = tokenize(&synopsis)
            .into_iter()
            .filter(|word| !STOP_WORDS.contains(word))
            .map(|word| Porter::stem(word))
            .collect::<Vec<_>>();

        let link = imdb.get_poster()?;
        let mut poster_file = File::create(format!("posters/{}.jpg", movie.imdb_id))?;
        let mut poster = reqwest::get(&link)?;
        poster.copy_to(&mut poster_file)?;

        let result = Movie {
            id: movie.imdb_id,
            cast: cast,
            crew: crew,
            title: movie.title,
            genres: movie.genres.into_iter().map(|genre| genre.name).collect(),
            keywords: keywords,
            popularity: movie.popularity,
            release_date: movie.release_date,
            revenue: movie.revenue,
            runtime: movie.runtime, 
            summary: summary,
            tokens: tokens,
            vote_average: movie.vote_average,
        };

        let mut movie_file = File::create(format!("movies/{}.json", result.id))?;
        movie_file.write_all(serde_json::to_string(&result)?.as_bytes())?;
        Ok(Index{
            id: result.id.clone(),
            title: result.title.clone(),
        })
    }
}
