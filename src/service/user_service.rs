use uuid::Uuid;

use crate::{
    config::db::Pool,
    models::{game::Game, user::{User, UserDto, UserFormDto}},
    response::{ErrorResponse, ResponseBody},
};

/// Queries the database and fetches the registered users in a game.
///
/// # Errors
///
/// This function fails if:
/// - an error occurred during execution.
/// - no game could be found with the given id.
///
pub fn find_by_game(game_id: Uuid, pool: &Pool) -> Result<Vec<UserDto>, ErrorResponse> {
    let game = Game::find_by_id(game_id, &mut pool.get().unwrap());
    if game.is_err() {
        return Err(ResponseBody::not_found_error(&format!(
            "Game with id '{}' not found",
            game_id.to_string()
        )));
    }

    match User::find_by_game(&game.unwrap(), &mut pool.get().unwrap()) {
        Ok(users) => {
            let result = users.into_iter()
                .map(|user| user.into())
                .collect::<Vec<UserDto>>();

            Ok(result)
        },
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
pub fn find_by_id(id: Uuid, pool: &Pool) -> Result<UserDto, ErrorResponse> {
    match User::find_by_id(id, &mut pool.get().unwrap()) {
        Ok(game) => Ok(game.into()),
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
pub fn insert(new_user: UserFormDto, pool: &Pool) -> Result<UserDto, ErrorResponse> {
    match User::insert(new_user, &mut pool.get().unwrap()) {
        Ok(score) => Ok(score.into()),
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
pub fn update(id: Uuid, updated_user: UserFormDto, pool: &Pool) -> Result<UserDto, ErrorResponse> {
    if !user_exists(id, pool) {
        return Err(ResponseBody::not_found_error(&format!(
            "User with id '{}' not found",
            id
        )));
    }

    match User::update(id, updated_user, &mut pool.get().unwrap()) {
        Ok(user) => Ok(user.into()),
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
