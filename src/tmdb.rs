use reqwest::get;
use serde_json::from_str;
use failure::Error;
use std::{thread, time, env};

/// Represents a movie from TMDB.
#[derive(Serialize)]
pub struct Movie {
    pub id: i32,
    pub imdb_id: String,
    pub cast: Vec<Cast>,
    pub crew: Vec<Crew>,
    pub title: String,
    pub genres: Vec<String>,
    pub keywords: Vec<String>,
    pub homepage: Option<String>, 
    pub popularity: Option<f32>,
    pub release_date: Option<String>,
    pub revenue: Option<f32>,
    pub runtime: Option<i32>, 
    pub tagline: Option<String>,
    pub vote_average: Option<f32>,
}

#[derive(Deserialize)]
struct MovieID {
    pub id: i32
}

#[derive(Deserialize)]
struct Credits {
    cast: Vec<Cast>,
    crew: Vec<Crew>,
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

#[derive(Deserialize)]
struct Genre {
    pub name: String,
}

#[derive(Deserialize)]
struct RawMovie {
    id: i32,
    imdb_id: String,
    title: String,
    genres: Vec<Genre>,
    homepage: Option<String>, 
    original_language: Option<String>,
    popularity: Option<f32>,
    release_date: Option<String>,
    revenue: Option<f32>,
    runtime: Option<i32>, 
    tagline: Option<String>,
    vote_average: Option<f32>,
}

impl RawMovie {
    pub fn is_english(&self) -> bool {
        Some("en".to_owned()) == self.original_language
    }
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
enum TMDBError {
    #[fail(display = "Non-English movie")]
    NonEnglish
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
            key: env::var("TMDB_API_KEY")
                .expect("Missing environment variable TMDB_API_KEY"),
        }
    }
}

impl TMDB {
    fn query(&mut self, url: &str) -> Result<String, Error> {
        let delta = time::Instant::now() - self.last_query;
        if *DELAY > delta { thread::sleep(delta); }
        let data = get(url)?.text()?;
        self.last_query = time::Instant::now();
        Ok(data)
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
    
    pub fn get_movie(&mut self, id: i32) -> Result<Movie, Error> {

        let m = self.get_raw_movie(id)?;
        let (cast, crew) = self.get_people(id)?;
        if !m.is_english() { Err(TMDBError::NonEnglish)? }

        Ok(
            Movie {
                id: m.id,
                imdb_id: m.imdb_id,
                cast: cast,
                crew: crew,
                title: m.title,
                genres: m.genres.into_iter().map(|genre| genre.name).collect(),
                keywords: self.get_keywords(id)?,
                homepage: m.homepage, 
                popularity: m.popularity,
                release_date: m.release_date,
                revenue: m.revenue,
                runtime: m.runtime, 
                tagline: m.tagline,
                vote_average: m.vote_average,
            } 
        )
    }

    fn get_raw_movie(&mut self, id: i32) -> Result<RawMovie, Error> {
        let url = format!(
            "https://api.themoviedb.org/3/movie/{}?api_key={}&language=en-US",
            id,
            &self.key
        );

        let data = self.query(&url)?;

        Ok(from_str(&data)?)
    }

    fn get_keywords(&mut self, id: i32) -> Result<Vec<String>, Error> {
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
