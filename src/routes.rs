use std::time::Duration;

use axum::{
    http::{
        header::{ACCEPT, AUTHORIZATION, CONTENT_DISPOSITION, CONTENT_TYPE},
        Method,
    },
    Router,
};
use tower_http::cors::{Any, CorsLayer};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::{controller, ApiDoc, SharedState};

/// Set up the entire routing for the web service and create the OpenAPI
/// documentation page
pub async fn create_app(state: SharedState) -> Router {
    Router::new()
        .merge(controller::api_routes())
        .merge(SwaggerUi::new("/swagger").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .layer(setup_cors())
        .with_state(state)
}

/// Creates a new [`CorsLayer`] to allow external origins making a call
/// to the server.
fn setup_cors() -> CorsLayer {
    CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::PATCH, Method::DELETE])
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE])
        .expose_headers([CONTENT_DISPOSITION])
        .max_age(Duration::from_secs(3600))
}
