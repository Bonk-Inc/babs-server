use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use utoipa::{OpenApi, ToSchema};
use uuid::Uuid;

use crate::{
    models::level::{Level, LevelForm},
    response::{ErrorResponse, ResponseBody},
    service::level_service,
    SharedState,
};

#[derive(OpenApi)]
#[openapi(
    paths(index, store, update, destroy),
    components(schemas(Level, LevelForm, LevelResponseBody, LevelsResponseBody))
)]
pub struct LevelApi;

/// The structure of the response body where there is a single level returned. This struct is primarily used for
/// the OpenAPI docs.
#[derive(ToSchema)]
pub struct LevelResponseBody {
    pub message: String,
    pub status: String,
    pub data: Level,
}

/// The structure of the response body where there are multiple levels returned. This struct is primarily used for
/// the OpenAPI docs.
#[derive(ToSchema)]
pub struct LevelsResponseBody {
    pub message: String,
    pub status: String,
    pub data: Vec<Level>,
}

#[utoipa::path(
    get,
    path = "/game/{gameId}",
    tag = "Level",
    operation_id = "level_games",
    params(
        ("gameId", Path, description = "Unique id of a Game"),
    ),
    responses(
        (status = StatusCode::OK, description = "Levels fetched by game successfully", body = LevelsResponseBody),
        (status = StatusCode::NOT_FOUND, description = "No Game found by game id", body = ErrorResponse)
    )
)]
pub async fn index(
    State(app_state): State<SharedState>,
    Path(game_id): Path<Uuid>,
) -> Result<ResponseBody<Vec<Level>>, ErrorResponse> {
    let pool = &app_state.read().unwrap().db;

    match level_service::find_by_game(game_id, pool) {
        Ok(levels) => Ok(ResponseBody::ok("Levels fetched", levels)),
        Err(err) => Err(err),
    }
}

#[utoipa::path(
    post,
    path = "",
    tag = "Level",
    operation_id = "level_store",
    request_body = LevelForm,
    responses(
        (status = StatusCode::CREATED, description = "Level created successfully", body = LevelsResponseBody),
        (status = StatusCode::BAD_REQUEST, description = "Invalid input", body = ErrorResponse)
    )
)]
pub async fn store(
    State(app_state): State<SharedState>,
    Json(new_level): Json<LevelForm>,
) -> Result<ResponseBody<Level>, ErrorResponse> {
    let pool = &app_state.read().unwrap().db;

    match level_service::insert(new_level, pool) {
        Ok(level) => Ok(ResponseBody::created("Level created", level)),
        Err(error) => Err(error),
    }
}

#[utoipa::path(
    put,
    path = "/{id}",
    tag = "Level",
    operation_id = "level_update",
    request_body = LevelForm,
    params(
        ("id", Path, description = "Unique id of a Level"),
    ),
    responses(
        (status = StatusCode::OK, description = "Level updated successfully", body = LevelsResponseBody),
        (status = StatusCode::BAD_REQUEST, description = "Invalid input", body = ErrorResponse),
        (status = StatusCode::NOT_FOUND, description = "No level found by id", body = ErrorResponse)
    )
)]
pub async fn update(
    State(app_state): State<SharedState>,
    Path(id): Path<Uuid>,
    Json(updated_level): Json<LevelForm>,
) -> Result<ResponseBody<Level>, ErrorResponse> {
    let pool = &app_state.read().unwrap().db;

    match level_service::update(id, updated_level, pool) {
        Ok(level) => Ok(ResponseBody::ok("Level updated", level)),
        Err(error) => Err(error),
    }
}

#[utoipa::path(
    delete,
    path = "/{id}",
    tag = "Level",
    operation_id = "level_destroy",
    params(
        ("id", Path, description = "Unique id of a Level"),
    ),
    responses(
        (status = StatusCode::NO_CONTENT, description = "Level deleted successfully"),
        (status = StatusCode::NOT_FOUND, description = "No level found by id", body = ErrorResponse)
    )
)]
pub async fn destroy(
    State(app_state): State<SharedState>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, ErrorResponse> {
    let pool = &app_state.read().unwrap().db;

    match level_service::delete(id, pool) {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(err) => Err(err),
    }
}
