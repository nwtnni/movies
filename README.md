# Movies

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

## Download Link

[Complete dataset extracted from IMDB and TMDB][1]

[1]: https://s3.us-east-2.amazonaws.com/cinema-pop-complete/data.tar
