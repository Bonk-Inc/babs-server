use uuid::Uuid;

use crate::{
    config::db::Pool,
    models::{
        game::{Game, GameDTO},
        level::{Level, LevelForm},
    },
    response::{ErrorResponse, ResponseBody},
};

/// Queries the database and fetches all the registered games.
///
/// # Errors
///
/// This function fails if:
/// - an error occurred during execution.
///
pub fn find_all(pool: &Pool) -> Result<Vec<Game>, ErrorResponse> {
    match Game::find_all(&mut pool.get().unwrap()) {
        Ok(games) => Ok(games),
        Err(_) => Err(ResponseBody::internal_error("Cannot fetch games")),
    }
}

/// Queries the database and fetches the registered game by the given id.
///
/// # Errors
///
/// This function fails if:
/// - an error occurred during execution.
/// - could not find game with given id.
///
pub fn find_by_id(id: Uuid, pool: &Pool) -> Result<Game, ErrorResponse> {
    match Game::find_by_id(id, &mut pool.get().unwrap()) {
        Ok(game) => Ok(game),
        Err(_) => Err(ResponseBody::not_found_error(&format!(
            "Game with id '{}' not found",
            id.to_string()
        ))),
    }
}

/// Inserts a new game object and into the database and adds a new level
/// to the game.
///
/// # Errors
///
/// This function fails if:
/// - an error occurred during execution.
///
pub fn insert(new_game: GameDTO, pool: &Pool) -> Result<Game, ErrorResponse> {
    match Game::insert(new_game, &mut pool.get().unwrap()) {
        Ok(game) => {
            let level = LevelForm {
                name: "Level 1".to_owned(),
                game_id: game.id,
            };

            match Level::insert(level, &mut pool.get().unwrap()) {
                Ok(_) => Ok(game),
                Err(_) => Err(ResponseBody::internal_error(
                    "Could not add level to newly created game",
                )),
            }
        }
        Err(_) => Err(ResponseBody::internal_error(
            "Could not add new game in database",
        )),
    }
}

/// Updates the game with the given id in the database.
///
/// # Errors
///
/// This function fails if:
/// - an error occurred during execution.
/// - no game could be found with the given id.
///
pub fn update(id: Uuid, updated_game: GameDTO, pool: &Pool) -> Result<Game, ErrorResponse> {
    if !game_exisits(id, pool) {
        return Err(ResponseBody::not_found_error(&format!(
            "Game with id '{}' not found",
            id.to_string()
        )));
    }

    match Game::update(id, updated_game, &mut pool.get().unwrap()) {
        Ok(game) => Ok(game),
        Err(_) => Err(ResponseBody::internal_error("Could not update game")),
    }
}

/// Deletes a game from the database with the given id.
///
/// # Errors
///
/// This function fails if:
/// - an error occurred during execution.
/// - no game could be found with the given id.
///
pub fn delete(id: Uuid, pool: &Pool) -> Result<usize, ErrorResponse> {
    if !game_exisits(id, pool) {
        return Err(ResponseBody::not_found_error(&format!(
            "Game with id '{}' not found",
            id.to_string()
        )));
    }

    match Game::delete(id, &mut pool.get().unwrap()) {
        Ok(result) => Ok(result),
        Err(_) => Err(ResponseBody::internal_error(
            "Error occurred when deleting game",
        )),
    }
}

/// Checks if a game exists in the database with the given id.
pub fn game_exisits(id: Uuid, pool: &Pool) -> bool {
    let game = Game::find_by_id(id, &mut pool.get().unwrap());

    game.is_ok()
}
