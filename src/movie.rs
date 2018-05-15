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
    pub original_language: String,
    pub rating: String,
    pub release_date: String,
    pub revenue: f32,
    pub runtime: i32, 
    pub summary: String,
    pub tokens: Vec<String>,
    pub tmdb_score_value: f32,
    pub tmdb_score_count: i32,
    pub imdb_score_value: f32,
    pub imdb_score_count: i32,
    pub meta_score_value: f32,
    pub meta_score_count: i32,
}

#[derive(Deserialize, Serialize)]
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

        let rating = imdb
            .get_rating()
            .unwrap_or_else(|_| {
                warn!("{} has no MPAA rating", movie.title.clone());
                "NOT RATED".to_owned()
            });

        let (imdb_score_value, imdb_score_count) = imdb.get_imdb_score()?;

        let (meta_score_value, meta_score_count) = imdb
            .get_metacritic_score()
            .unwrap_or_else(|_| {
                warn!("{} has no Metacritic ratings", movie.title.clone());
                (0., 0)
            });

        let summary = imdb
            .get_summary()
            .or_else(|error| {
                match &movie.overview {
                | &None => Err(IMDBError::Summary { name: movie.title.clone() }),
                | &Some(ref text) => {
                    if text.is_empty() {
                        Err(IMDBError::Summary { name: movie.title.clone() })
                    } else {
                        warn!("{}; substituting overview of length {}", error, text.len());
                        Ok(text.to_owned())
                    }
                }
                }
            })?;

        let synopsis = imdb
            .get_synopsis()
            .and_then(|text| {
                info!("[TOKENS] {}: synopsis of length {}", id, text.len());
                Ok(text)
            }).unwrap_or_else(|_| {
                warn!("[TOKENS] {}: summary of length {}", id, summary.len());
                summary.clone()
            });

        let tokens = tokenize(&synopsis)
            .into_iter()
            .filter(|word| !STOP_WORDS.contains(word))
            .map(|word| Porter::stem(word))
            .collect::<Vec<_>>();

        let link = imdb.get_poster()?;
        let mut poster_file = File::create(format!("posters/{}.jpg", movie.imdb_id))?;
        let mut poster = reqwest::get(&link)?;
        poster.copy_to(&mut poster_file)?;

        let mut movie_file = File::create(format!("movies/{}.json", movie.imdb_id))?;
        movie_file.write_all(
            serde_json::to_string(
                &Movie {
                    id: movie.imdb_id.clone(),
                    cast,
                    crew,
                    title: movie.title.clone(),
                    genres: movie.genres.into_iter().map(|genre| genre.name).collect(),
                    keywords,
                    original_language: movie.original_language,
                    rating,
                    release_date: movie.release_date,
                    revenue: movie.revenue,
                    runtime: movie.runtime, 
                    summary,
                    tokens,
                    tmdb_score_value: movie.vote_average,
                    tmdb_score_count: movie.vote_count,
                    imdb_score_value,
                    imdb_score_count,
                    meta_score_value,
                    meta_score_count,
                }
            )?.as_bytes()
        )?;

        Ok(Index{
            id: movie.imdb_id,
            title: movie.title,
        })
    }
}
