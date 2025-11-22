use axum::{routing::{get, post, put}, Router};

use crate::SharedState;

pub mod game;
pub mod level;
pub mod score;
pub mod stats;
pub mod user;

pub fn game_routes() -> Router<SharedState> {
    Router::new()
        .route("/", get(game::index).post(game::store))
        .route("/{gameId}", get(game::show).put(game::update).delete(game::destroy))
}

pub fn level_routes() -> Router<SharedState> {
    Router::new()
        .route("/", post(level::store))
        .route("/game/{gameId}", get(level::index))
        .route("/{levelId}", put(level::update).delete(level::destroy))
}

pub fn score_routes() -> Router<SharedState> {
    Router::new()
        .route("/", post(score::store))
        .route("/game/{gameId}", get(score::index))
        .route("/{scoreId}", get(score::show).put(score::update).delete(score::destroy))
        .route("/level/{levelId}", get(score::level_scores))
        .route("/user/{userId}", get(score::user_scores))
}

pub fn stats_routes() -> Router<SharedState> {
    Router::new()
        .route("/all", get(stats::all))
        .route("/game/{gameId}", get(stats::game_stats))
}

pub fn user_routes() -> Router<SharedState> {
    Router::new()
        .route("/", post(user::store))
        .route("/game/{gameId}", get(user::index))
        .route("/{userId}", put(user::update).delete(user::destroy))
}

pub async fn healthcheck() -> &'static str {
    "Ok"
}