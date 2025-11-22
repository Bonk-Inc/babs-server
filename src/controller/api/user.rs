use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use utoipa::{OpenApi, ToSchema};
use uuid::Uuid;

use crate::{
    models::user::{User, UserForm},
    response::{ErrorResponse, ResponseBody},
    service::user_service,
    SharedState,
};

#[derive(OpenApi)]
#[openapi(
    paths(index, store, update, destroy),
    components(schemas(User, UserForm, UserResponseBody, UsersResponseBody))
)]
pub struct UserApi;

/// The structure of the response body where there is a single returned. This struct is primarily used for
/// the OpenAPI docs.
#[derive(ToSchema)]
pub struct UsersResponseBody {
    pub message: String,
    pub status: String,
    pub data: User,
}

/// The structure of the response body where there are multiple users returned. This struct is primarily used for
/// the OpenAPI docs.
#[derive(ToSchema)]
pub struct UserResponseBody {
    pub message: String,
    pub status: String,
    pub data: Vec<User>,
}

#[utoipa::path(
    get,
    path = "/game/{id}",
    tag = "User",
    operation_id = "user_index",
    params(
        ("id", Path, description = "Unique id of a game"),
    ),
    responses(
        (status = StatusCode::OK, description = "User fetched successfully", body = UsersResponseBody)
    )
)]
pub async fn index(
    State(app_state): State<SharedState>,
    Path(game_id): Path<Uuid>,
) -> Result<ResponseBody<Vec<User>>, ErrorResponse> {
    let pool = &app_state.read().unwrap().db;

    match user_service::find_by_game(game_id, pool) {
        Ok(users) => Ok(ResponseBody::ok("Users fetched", users)),
        Err(err) => Err(err),
    }
}

#[utoipa::path(
    post,
    path = "",
    tag = "User",
    operation_id = "user_store",
    request_body = UserForm,
    responses(
        (status = StatusCode::CREATED, description = "New user created", body = UsersResponseBody),
        (status = StatusCode::BAD_REQUEST, description = "Invalid input", body = ErrorResponse)
    )
)]
pub async fn store(
    State(app_state): State<SharedState>,
    Json(new_user): Json<UserForm>,
) -> Result<ResponseBody<User>, ErrorResponse> {
    let pool = &app_state.read().unwrap().db;

    match user_service::insert(new_user, pool) {
        Ok(added_user) => Ok(ResponseBody::created("User created", added_user)),
        Err(err) => Err(err),
    }
}

#[utoipa::path(
    put,
    path = "/{id}",
    tag = "User",
    operation_id = "user_update",
    request_body = UserForm,
    params(
        ("id", Path, description = "Unique id of a user"),
    ),
    responses(
        (status = StatusCode::OK, description = "User updated successfully", body = UserResponseBody),
        (status = StatusCode::BAD_REQUEST, description = "Invalid input", body = ErrorResponse),
        (status = StatusCode::NOT_FOUND, description = "No user found by id", body = ErrorResponse)
    )
)]
pub async fn update(
    State(app_state): State<SharedState>,
    Path(id): Path<Uuid>,
    Json(updated_user): Json<UserForm>,
) -> Result<ResponseBody<User>, ErrorResponse> {
    let pool = &app_state.read().unwrap().db;

    match user_service::update(id, updated_user, pool) {
        Ok(level) => Ok(ResponseBody::ok("User updated", level)),
        Err(error) => Err(error),
    }
}

#[utoipa::path(
    delete,
    path = "/{id}",
    tag = "User",
    operation_id = "user_destroy",
    params(
        ("id", Path, description = "Unique id of a user"),
    ),
    responses(
        (status = StatusCode::NO_CONTENT, description = "User deleted successfully"),
        (status = StatusCode::NOT_FOUND, description = "No user found by id", body = ErrorResponse)
    )
)]
pub async fn destroy(
    State(app_state): State<SharedState>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, ErrorResponse> {
    let pool = &app_state.read().unwrap().db;

    match user_service::delete(id, pool) {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(err) => Err(err),
    }
}
