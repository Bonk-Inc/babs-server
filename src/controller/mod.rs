use axum::{middleware, routing::get, Router};

use crate::{middleware::auth_middleware, SharedState};

pub mod api;

pub fn api_routes() -> Router<SharedState> {
    Router::new()
        .nest("/game", api::game_routes())
        .nest("/level", api::level_routes())
        .nest("/stats", api::stats_routes())
        .layer(middleware::from_fn(auth_middleware::verify_token))
        .nest("/score", api::score_routes())
        .nest("/user", api::user_routes())
        .route("/healthcheck", get(api::healthcheck))
}