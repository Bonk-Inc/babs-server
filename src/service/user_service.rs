use uuid::Uuid;

use crate::{
    config::db::Pool,
    models::user::{User, UserForm},
    response::{ErrorResponse, ResponseBody},
};

use super::game_service;

/// Queries the database and fetches the registered users in a game.
///
/// # Errors
///
/// This function fails if:
/// - an error occurred during execution.
/// - no game could be found with the given id.
///
pub fn find_by_game(game_id: Uuid, pool: &Pool) -> Result<Vec<User>, ErrorResponse> {
    let game = game_service::find_by_id(game_id, pool);
    if game.is_err() {
        return Err(ResponseBody::not_found_error(&format!(
            "Game with id '{}' not found",
            game_id.to_string()
        )));
    }

    match User::find_by_game(&game?, &mut pool.get().unwrap()) {
        Ok(users) => Ok(users),
        Err(_) => Err(ResponseBody::internal_error("Cannot fetch users")),
    }
}

/// Queries the database and fetches the registered user with id.
///
/// # Errors
///
/// This function fails if:
/// - an error occurred during execution.
/// - no user could be found with the given id.
///
pub fn find_by_id(id: Uuid, pool: &Pool) -> Result<User, ErrorResponse> {
    match User::find_by_id(id, &mut pool.get().unwrap()) {
        Ok(game) => Ok(game),
        Err(_) => Err(ResponseBody::not_found_error(&format!(
            "User with id '{}' not found",
            id.to_string()
        ))),
    }
}

/// Inserts a new user into the database.
///
/// # Errors
///
/// This function fails if:
/// - an error occurred during execution.
///
pub fn insert(new_user: UserForm, pool: &Pool) -> Result<User, ErrorResponse> {
    match User::insert(new_user, &mut pool.get().unwrap()) {
        Ok(score) => Ok(score),
        Err(err) => Err(ResponseBody::internal_error(&format!(
            "Error saving new user, {}",
            err
        ))),
    }
}

/// Updates a user in the database with the given id.
///
/// # Errors
///
/// This function fails if:
/// - an error occurred during execution.
/// - no user could be found with the given id.
///
pub fn update(id: Uuid, updated_user: UserForm, pool: &Pool) -> Result<User, ErrorResponse> {
    if !user_exists(id, pool) {
        return Err(ResponseBody::not_found_error(&format!(
            "User with id '{}' not found",
            id
        )));
    }

    match User::update(id, updated_user, &mut pool.get().unwrap()) {
        Ok(level) => Ok(level),
        Err(_) => Err(ResponseBody::internal_error("Could not update user")),
    }
}

/// Deletes a user in the database with the given id.
///
/// # Errors
///
/// This function fails if:
/// - an error occurred during execution.
/// - no user could be found with the given id.
///
pub fn delete(id: Uuid, pool: &Pool) -> Result<usize, ErrorResponse> {
    if !user_exists(id, pool) {
        return Err(ResponseBody::not_found_error(&format!(
            "User with id '{}' not found",
            id.to_string()
        )));
    }

    match User::delete(id, &mut pool.get().unwrap()) {
        Ok(results) => Ok(results),
        Err(_) => Err(ResponseBody::internal_error("Could not delete user")),
    }
}

/// Checks if a user exists in the database with the given id.
pub fn user_exists(id: Uuid, pool: &Pool) -> bool {
    User::find_by_id(id, &mut pool.get().unwrap()).is_ok()
}
