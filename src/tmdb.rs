use reqwest::get;
use serde_json::from_str;
use failure::Error;
use std::{thread, time, env};

use movie::*;

#[derive(Deserialize)]
struct MovieID {
    pub id: i32
}

#[derive(Deserialize)]
struct Credits {
    cast: Vec<Cast>,
    crew: Vec<Crew>,
}

#[derive(Deserialize)]
pub struct Genre {
    pub name: String,
}

#[derive(Deserialize)]
pub struct RawMovie {
    pub id: i32,
    pub imdb_id: String,
    pub title: String,
    pub adult: bool,
    pub genres: Vec<Genre>,
    pub original_language: String,
    pub overview: Option<String>,
    pub release_date: String,
    pub revenue: f32,
    pub runtime: i32, 
    pub vote_average: f32,
    pub vote_count: i32,
}

#[derive(Deserialize)]
struct Keyword {
    pub name: String
}

#[derive(Deserialize)]
struct Keywords {
    pub keywords: Vec<Keyword>
}

#[derive(Deserialize)]
struct Page {
    pub total_pages: i32,
    pub results: Vec<MovieID>,
}

#[derive(Debug, Fail)]
pub enum TMDBError {
    #[fail(display = "{} is an adult movie", name)]
    Adult { name: String },

    #[fail(display = "{}: is missing runtime", name)]
    Runtime { name: String },
}

lazy_static! {
    static ref DELAY: time::Duration = time::Duration::from_millis(250);
}

/// Represents a TMDB connection with API key and rate limiting.
pub struct TMDB {
    last_query: time::Instant,
    key: String,
}

impl Default for TMDB {
    fn default() -> Self {
        TMDB {
            last_query: time::Instant::now(),
            key: env::var("TMDB_API_KEY").expect("Missing environment variable TMDB_API_KEY"),
        }
    }
}

impl TMDB {
    pub fn query(&mut self, url: &str) -> Result<String, Error> {
        let delta = time::Instant::now() - self.last_query;
        if *DELAY > delta { thread::sleep(delta); }
        let data = get(url)?.text()?;
        self.last_query = time::Instant::now();
        Ok(data)
    }

    pub fn get_latest(&mut self) -> Result<i32, Error> {
        let url = format!(
            "https://api.themoviedb.org/3/movie/latest?api_key={}&language=en-US",
            &self.key,
        );

        let data = self.query(&url)?;
        let movie: MovieID = from_str(&data)?;
        Ok(movie.id)
    }

    pub fn get_page(&mut self, page: i32) -> Result<Vec<i32>, Error> {
        let url = format!(
            "https://api.themoviedb.org/3/movie/popular?api_key={}&language=en-US&page={}",
            &self.key,
            page
        );

        let data = self.query(&url)?;

        Ok(
            from_str::<Page>(&data)?
                .results
                .into_iter()
                .map(|movie| movie.id)
                .collect()
        )
    }

    pub fn get_raw_movie(&mut self, id: i32) -> Result<RawMovie, Error> {
        let url = format!(
            "https://api.themoviedb.org/3/movie/{}?api_key={}&language=en-US",
            id,
            &self.key
        );

        let data = self.query(&url)?;
        let movie: RawMovie = from_str(&data)?;

        if movie.adult {
            Err(TMDBError::Adult { name: movie.title })?
        } else if movie.runtime == 0 {
            Err(TMDBError::Runtime { name: movie.title })?
        } else {
            Ok(movie)
        }
    }

    pub fn get_keywords(&mut self, id: i32) -> Result<Vec<String>, Error> {
        let url = format!(
            "https://api.themoviedb.org/3/movie/{}/keywords?api_key={}&language=en-US",
            id,
            &self.key
        );

        let data = self.query(&url)?;

        Ok(
            from_str::<Keywords>(&data)?.keywords
                .into_iter()
                .map(|word| word.name)
                .collect()
        )
    }

    pub fn get_people(&mut self, id: i32) -> Result<(Vec<Cast>, Vec<Crew>), Error> {
        let url = format!(
            "https://api.themoviedb.org/3/movie/{}/credits?api_key={}&language=en-US",
            id,
            &self.key
        );

        let data = self.query(&url)?;
        let mut credits = from_str::<Credits>(&data)?;

        credits.crew.retain(|crew| {
            match crew.job.as_ref() {
            | "Director" | "Producer" | "Writer" | "Screenplay" | "Original Music Composer" => true,
            | _ => false,
            }                
        });

        credits.cast.retain(|cast| !cast.character.is_empty() && !cast.name.is_empty());
        credits.cast.truncate(20);
        return Ok((credits.cast, credits.crew));
    }
}
