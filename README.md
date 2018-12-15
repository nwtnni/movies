# Movies

A utility for scraping movie posters and data from [IMDB][1] and [TMDB][2]. Used to
collect data for [CinemaPop][3], our [final project][4] for CS 4300: Language and Information.

Requires a [TMDB API key][5] loaded in the environment variable `TMDB_API_KEY`.

NOTE: relies on HTML scraping via CSS selectors, and is fairly brittle. Modifications
may need to be made to the `imdb` module if IMDB updates their HTML layout.

## Data Format

### movies.json

```json
[
  {
    "id"    : "IMDB_ID_0",
    "title" : "MOVIE_TITLE_0",
  },
  {
    "id"    : "IMDB_ID_1",
    "title" : "MOVIE_TITLE_1",
  },
  {
    "id"    : "IMDB_ID_N",
    "title" : "MOVIE_TITLE_N",
  }
]
```

### movies

```json
{
  "id"                : "IMDB_ID",

  "cast"              : [
                          {
                            "character" : "CHARACTER_NAME_1",
                            "name"      : "ACTOR_NAME_1"
                          },
                          {
                            "character" : "CHARACTER_NAME_1",
                            "name"      : "ACTOR_NAME_1"
                          }
                        ],

  "crew"              : [
                          {
                            "job"  : "JOB_NAME_0",
                            "name" : "CREW_NAME_0"
                          },
                          {
                            "job"  : "JOB_NAME_1",
                            "name" : "CREW_NAME_1"
                          }
                        ],

  "title"             : "MOVIE_TITLE",

  "genres"            : [
                          "GENRE_0",
                          "GENRE_1"
                        ],

  "keywords"          : [
                          "KEYWORD_0",
                          "KEYWORD_1",
                        ],

  "original_language" : "LANGUAGE",

  "rating"            : "MPAA_RATING",

  "release_date"      : "RELEASE_DATE",

  "revenue"           : 0.0,

  "runtime"           : 0,

  "summary"           : "SUMMARY_TEXT",

  "tokens"            : [
                          "SYNOPSIS_TOKEN_0",
                          "SYNOPSIS_TOKEN_1"
                        ],

  "tmdb_score_value" : 0.0,

  "tmdb_score_count" : 0,

  "imdb_score_value" : 0.0,

  "imdb_score_count" : 0,

  "meta_score_value" : 0.0,

  "meta_score_count" : 0,
}
```

### posters

```
IMDB_ID_0.jpg
IMDB_ID_1.jpg
...
IMDB_ID_N.jpg
```

## Visualizations

NOTE: this is the subset of movies with non-zero Metacritic rating counts, used as a rough metric of movie quality.

## Genres

![Genre distribution](/resources/genres.png)

## Original Languages

![Language distribution](/resources/languages.png)

## MPAA Ratings

![Rating distribution](/resources/ratings.png)

## Runtime

![Runtime distribution](/resources/runtime.png)

## Scores

![Score distribution](/resources/scores.png)

## Token Counts

![Token distribution](/resources/tokens.png)
![Medium token distribution](/resources/med_tokens.png)
![Small token distribution](/resources/small_tokens.png)

[1]: https://www.imdb.com/
[2]: https://www.themoviedb.org/
[3]: http://cinemapop.infosci.cornell.edu/
[4]: https://github.com/nwtnni/cinema-pop
[5]: https://www.themoviedb.org/faq/api
