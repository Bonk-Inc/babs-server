use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use utoipa::{OpenApi, ToSchema};
use uuid::Uuid;

use crate::{
    models::game::{GameDto, GameFormDto},
    response::{ErrorResponse, ResponseBody},
    service::game_service,
    SharedState,
};

#[derive(OpenApi)]
#[openapi(
    paths(index, show, store, update, destroy),
    components(schemas(GameDto, GameFormDto, GameResponseBody, GamesResponseBody))
)]
pub struct GameApi;

/// The structure of the response body where there is a single games returned. This struct is primarily used for
/// the OpenAPI docs.
#[derive(ToSchema)]
pub struct GameResponseBody {
    pub message: String,
    pub status: String,
    pub data: GameDto,
}

/// The structure of the response body where there are multiple games returned. This struct is primarily used for
/// the OpenAPI docs.
#[derive(ToSchema)]
pub struct GamesResponseBody {
    pub message: String,
    pub status: String,
    pub data: Vec<GameDto>,
}

#[utoipa::path(
    get,
    path = "",
    tag = "Game",
    operation_id = "game_index",
    description = "Returns all the registered Games",
    responses(
        (status = StatusCode::OK, description = "Games fetched successfully", body = GamesResponseBody)
    )
)]
pub async fn index(
    State(app_state): State<SharedState>,
) -> Result<ResponseBody<Vec<GameDto>>, ErrorResponse> {
    let pool = &app_state.read().unwrap().db;

    match game_service::find_all(pool) {
        Ok(games) => {
            let body = games.into_iter()
                .map(|game| game.into())
                .collect::<Vec<GameDto>>();

            Ok(ResponseBody::ok("Games fetched", body))
        },
        Err(err) => Err(err),
    }
}

#[utoipa::path(
    get,
    path = "/{id}",
    tag = "Game",
    operation_id = "game_show",
    description = "Gets a Game with the given id",
    params(
        ("id", Path, description = "Unique id of a Game")
    ),
    description = "Returns the Game with the given id",
    responses(
        (status = StatusCode::OK, description = "Gam fetched successfully", body = GameResponseBody),
        (status = StatusCode::NOT_FOUND, description = "No game found by id", body = ErrorResponse)
    )
)]
pub async fn show(
    State(app_state): State<SharedState>,
    Path(id): Path<Uuid>,
) -> Result<ResponseBody<GameDto>, ErrorResponse> {
    let pool = &app_state.read().unwrap().db;

    match game_service::find_by_id(id, pool) {
        Ok(game) => Ok(ResponseBody::ok("Game fetched", game.into())),
        Err(err) => Err(err),
    }
}

#[utoipa::path(
    post,
    path = "",
    tag = "Game",
    operation_id = "game_store",
    request_body = GameFormDto,
    description = "Creates a new Game",
    responses(
        (status = StatusCode::CREATED, description = "Game created successfully", body = GameResponseBody),
        (status = StatusCode::BAD_REQUEST, description = "Invalid input", body = ErrorResponse)
    )
)]
pub async fn store(
    State(app_state): State<SharedState>,
    Json(new_game): Json<GameFormDto>,
) -> Result<ResponseBody<GameDto>, ErrorResponse> {
    let pool = &app_state.read().unwrap().db;

    match game_service::insert(new_game, pool) {
        Ok(game) => Ok(ResponseBody::created("Game created", game.into())),
        Err(err) => Err(err),
    }
}

#[utoipa::path(
    put,
    path = "/{id}",
    tag = "Game",
    operation_id = "game_update",
    request_body = GameFormDto,
    description = "Updates the Game with the given id",
    params(
        ("id", Path, description = "Unique id of a Game")
    ),
    responses(
        (status = StatusCode::OK, description = "Game updated successfully", body = GameResponseBody),
        (status = StatusCode::BAD_REQUEST, description = "Invalid input", body = ErrorResponse),
        (status = StatusCode::NOT_FOUND, description = "No game found by id", body = ErrorResponse)
    )
)]
pub async fn update(
    State(app_state): State<SharedState>,
    Path(id): Path<Uuid>,
    Json(updated_game): Json<GameFormDto>,
) -> Result<ResponseBody<GameDto>, ErrorResponse> {
    let pool = &app_state.read().unwrap().db;

    match game_service::update(id, updated_game, pool) {
        Ok(game) => Ok(ResponseBody::ok("Game updated", game.into())),
        Err(err) => Err(err),
    }
}

#[utoipa::path(
    delete,
    path = "/{id}",
    tag = "Game",
    operation_id = "game_destroy",
    description = "Deletes the Game with the given id",
    params(
        ("id", Path, description = "Unique id of a Game")
    ),
    responses(
        (status = StatusCode::NO_CONTENT, description = "Game deleted successfully"),
        (status = StatusCode::NOT_FOUND, description = "No score found by id", body = ErrorResponse)
    )
)]
pub async fn destroy(
    State(app_state): State<SharedState>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, ErrorResponse> {
    let pool = &app_state.read().unwrap().db;

    match game_service::delete(id, pool) {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(err) => Err(err),
    }
}
