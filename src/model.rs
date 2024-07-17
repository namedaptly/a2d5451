use serde::{Deserialize, Serialize};

pub(crate) type MovieId = String;

#[derive(Clone, Debug)]
// Internal representation - we want this to be separate from the models used by API layer
pub(crate) struct Movie {
    pub(crate) name: String,
    pub(crate) year: u16,
    pub(crate) was_good: bool,
}

impl From<PostMovieRequest> for Movie {
    fn from(value: PostMovieRequest) -> Self {
        return Movie {
            name: value.name,
            year: value.year,
            was_good: value.was_good,
        };
    }
}

// The server is expected to generate a string id for the movie.
#[derive(Clone, Debug, Deserialize)]
pub(crate) struct PostMovieRequest {
    pub(crate) name: String,
    pub(crate) year: u16,
    pub(crate) was_good: bool,
}

// Explicitly not tied to internal representation of Movie
#[derive(Debug, Serialize)]
pub(crate) struct PostMovieResponse {
    pub(crate) id: MovieId,
}

impl From<MovieId> for PostMovieResponse {
    fn from(value: MovieId) -> Self {
        PostMovieResponse { id: value }
    }
}

#[derive(Debug, Deserialize)]
pub(crate) struct GetMovieRequest {
    pub(crate) id: MovieId,
}

#[derive(Debug, Serialize)]
pub(crate) struct GetMovieResponse {
    pub(crate) name: String,
    pub(crate) year: u16,
    pub(crate) was_good: bool,
}

impl From<Movie> for GetMovieResponse {
    fn from(value: Movie) -> Self {
        return GetMovieResponse {
            name: value.name,
            year: value.year,
            was_good: value.was_good,
        };
    }
}
