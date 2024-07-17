use std::{
    collections::HashMap,
    hash::{DefaultHasher, Hash, Hasher},
    sync::Arc,
};

use axum::{
    extract::{rejection::JsonRejection, Path, State},
    routing::{get, post},
    Json, Router,
};

use error::Error;
use model::{
    GetMovieRequest, GetMovieResponse, Movie, MovieId, PostMovieRequest, PostMovieResponse,
};
use tokio::sync::Mutex;

// Create Axum server with the following endpoints:
// 1. GET /movie/{id} - This should return back a movie given the id
// 2. POST /movie - this should save movie in a DB (HashMap<String, Movie>). This movie will be sent
// via a JSON payload.
// As a bonus: implement a caching layer so we don't need to make expensive "DB" lookups, etc.

// Assumptions
// 1. Server generates the movie id
// 2. If the same movie is POST'ed more than once, the ID remains the same (based on the movie title for now)
//    but the value is overwritten (because we have no delete)

mod error;
mod model;

struct AppState {
    db: Mutex<HashMap<MovieId, Movie>>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let appstate = Arc::new(AppState {
        db: Mutex::new(HashMap::default()),
    });

    // build our application with a single route
    let app = Router::new()
        .route("/movie/:id", get(get_movie))
        .route("/movie/", post(post_movie))
        .with_state(appstate.clone());

    // run our app with hyper, listening globally on port 8080
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await?;
    axum::serve(listener, app).await?;

    Ok(())
}

async fn get_movie(
    State(state): State<Arc<AppState>>,
    Path(request): Path<GetMovieRequest>,
) -> Result<Json<GetMovieResponse>, Error> {
    if let Some(movie) = state.db.lock().await.get(&request.id) {
        Ok(Json(movie.clone().into()))
    } else {
        Err(Error::MovieNotFound(request.id))
    }
}

async fn post_movie(
    State(state): State<Arc<AppState>>,
    payload: Result<Json<PostMovieRequest>, JsonRejection>,
) -> Result<Json<PostMovieResponse>, Error> {
    match payload {
        Ok(payload) => Ok(create_movie(&state, payload).await),
        Err(e) => Err(Error::FailedToParseRequest(e.to_string())),
    }
}

// For now infallible, but this would return a result in real life
async fn create_movie(
    state: &Arc<AppState>,
    Json(req): Json<PostMovieRequest>,
) -> Json<PostMovieResponse> {
    let key = generate_new_movie_id(&req.name);
    state
        .db
        .lock()
        .await
        .entry(key.clone())
        .and_modify(|v| *v = req.clone().into())
        .or_insert(req.into()); // borrow checker doesn't realize only one of these will be called
    Json(key.into())
}

// A not-so-fancy way of converting a name to hashed value.  For a real DB, there might be
// surrogate primary keys or this hash value may end up being the primary key.
fn generate_new_movie_id(key: &str) -> MovieId {
    let mut hasher = DefaultHasher::new();
    key.hash(&mut hasher);
    hasher.finish().to_string()
}
