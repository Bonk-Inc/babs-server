use uuid::Uuid;

use crate::{
    config::db::Pool,
    models::level::{Level, LevelForm},
    response::{ErrorResponse, ResponseBody},
};

use super::game_service;

/// Queries the database and fetches all the registered levels.
///
/// # Errors
///
/// This function fails if:
/// - an error occurred during execution.
///
pub fn find_all(pool: &Pool) -> Result<Vec<Level>, ErrorResponse> {
    match Level::find_all(&mut pool.get().unwrap()) {
        Ok(levels) => Ok(levels),
        Err(_) => Err(ResponseBody::internal_error("Cannot fetch levels")),
    }
}

/// Queries the database and fetches the registered level by the given id.
///
/// # Errors
///
/// This function fails if:
/// - an error occurred during execution.
/// - could not find level with given id.
///
pub fn find_by_id(id: Uuid, pool: &Pool) -> Result<Level, ErrorResponse> {
    match Level::find_by_id(id, &mut pool.get().unwrap()) {
        Ok(level) => Ok(level),
        Err(_) => Err(ResponseBody::not_found_error(&format!(
            "Level with id '{}' not found",
            id.to_string()
        ))),
    }
}

/// Queries the database and fetches the registered levels by the given game.
///
/// # Errors
///
/// This function fails if:
/// - could not find game with given id.
/// - an error occurred during execution.
///
pub fn find_by_game(game_id: Uuid, pool: &Pool) -> Result<Vec<Level>, ErrorResponse> {
    let game = game_service::find_by_id(game_id, pool);
    if game.is_err() {
        return Err(ResponseBody::not_found_error(&format!(
            "Game with id '{}' not found",
            game_id.to_string()
        )));
    }

    match Level::find_by_game(&game?, &mut pool.get().unwrap()) {
        Ok(levels) => Ok(levels),
        Err(_) => Err(ResponseBody::internal_error(
            "Cannot add a new level in database",
        )),
    }
}

/// Inserts a new level object and into the database.
///
/// # Errors
///
/// This function fails if:
/// - an error occurred during execution.
///
pub fn insert(new_level: LevelForm, pool: &Pool) -> Result<Level, ErrorResponse> {
    match Level::insert(new_level, &mut pool.get().unwrap()) {
        Ok(level) => Ok(level),
        Err(_) => Err(ResponseBody::internal_error(
            "Cannot add a new level in database",
        )),
    }
}

/// Updates the level with the given id in the database.
///
/// # Errors
///
/// This function fails if:
/// - an error occurred during execution.
/// - no level could be found with the given id.
///
pub fn update(id: Uuid, updated_level: LevelForm, pool: &Pool) -> Result<Level, ErrorResponse> {
    if !level_exists(id, pool) {
        return Err(ResponseBody::not_found_error(&format!(
            "Level with id '{}' not found",
            id.to_string()
        )));
    }

    match Level::update(id, updated_level, &mut pool.get().unwrap()) {
        Ok(level) => Ok(level),
        Err(_) => Err(ResponseBody::internal_error("Could not update level")),
    }
}

/// Deletes a level from the database with the given id.
///
/// # Errors
///
/// This function fails if:
/// - an error occurred during execution.
/// - no level could be found with the given id.
///
pub fn delete(id: Uuid, pool: &Pool) -> Result<usize, ErrorResponse> {
    if !level_exists(id, pool) {
        return Err(ResponseBody::not_found_error(&format!(
            "Level with id '{}' not found",
            id.to_string()
        )));
    }

    match Level::delete(id, &mut pool.get().unwrap()) {
        Ok(results) => Ok(results),
        Err(_) => Err(ResponseBody::internal_error("Could not delete level")),
    }
}

/// Checks if a level exists in the database with the given id.
pub fn level_exists(id: Uuid, pool: &Pool) -> bool {
    Level::find_by_id(id, &mut pool.get().unwrap()).is_ok()
}
