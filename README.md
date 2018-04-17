# Movies

## Data Format

### movies.json

```json
[
  {
    "id"    : "IMDB_ID_0",
    "title" : "MOVIE_TITLE_0",
  },

  ...

  {
    "id"    : "IMDB_ID_N",
    "title" : "MOVIE_TITLE_N",
  },
]
```

### movies

```json
{
  "id"    : "IMDB_ID",
  "cast" : [
    {
      "character" : "CHARACTER_NAME",
      "name"      : "ACTOR_NAME"
    }
  ],
  "crew" : [
    {
      "job"  : "JOB_NAME",
      "name" : "CREW_NAME"
    } 
  ]
  "title" : "MOVIE_TITLE",
  "genres" : [
    "GENRE_0",
    "GENRE_1"
  ],
  "keywords" : [
    "KEYWORD_0",
    "KEYWORD_1",
  ],
  "original_language" : "LANGUAGE",
  "rating" : "MPAA_RATING",
  "release_date" : "RELEASE_DATE",
  "revenue" : REVENUE_FLOAT,
  "runtime" : RUNTIME_INT,
  "summary" : "SUMMARY_TEXT",
  "tokens" : [
    "SYNOPSIS_TOKEN_0",
    "SYNOPSIS_TOKEN_1"
  ],
  "tmdb_score_value" : TMDB_SCORE_VALUE_FLOAT,
  "tmdb_score_count" : TMDB_SCORE_COUNT_INT,
  "imdb_score_value" : IMDB_SCORE_VALUE_FLOAT,
  "imdb_score_count" : IMDB_SCORE_COUNT_INT,
  "meta_score_value" : META_SCORE_VALUE_FLOAT,
  "meta_score_count" : META_SCORE_COUNT_INT,
}
```

### posters

```
IMDB_ID_0.jpg
IMDB_ID_1.jpg
...
IMDB_ID_N.jpg
```

## Download Link

[Complete dataset extracted from IMDB and TMDB][1]

[1]: https://s3.us-east-2.amazonaws.com/cinema-pop-complete/data.tar
