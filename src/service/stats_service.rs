use uuid::Uuid;

use crate::{
    config::db::Pool,
    models::{game::Game, score::Score, user::User},
    response::{ErrorResponse, ResponseBody},
};

use super::game_service;

/// Queries the database and counts the registered games.
///
/// # Errors
///
/// This function fails if:
/// - an error occured during execution.
///
pub fn count_games(pool: &Pool) -> Result<i64, ErrorResponse> {
    match Game::count(&mut pool.get().unwrap()) {
        Ok(count) => Ok(count),
        Err(_) => Err(ResponseBody::internal_error(
            "Cannot count games in database",
        )),
    }
}

/// Queries the database and counts all the registered scores. If a game is defined, only the
/// related score of the game will be counted.
///
/// # Errors
///
/// This function fails if:
/// - no game was found with the given id.
/// - an error occurred during execution.
///
pub fn count_scores(game_id: Option<Uuid>, pool: &Pool) -> Result<i64, ErrorResponse> {
    let mut game: Option<Game> = None;
    if let Some(id) = game_id {
        let fetched_game = game_service::find_by_id(id, pool);
        if fetched_game.is_err() {
            return Err(ResponseBody::not_found_error(&format!(
                "Game with id '{}' not found",
                id.to_string()
            )));
        }

        game = Some(fetched_game?)
    }

    match Score::count(&game, &mut pool.get().unwrap()) {
        Ok(count) => Ok(count),
        Err(_) => Err(ResponseBody::internal_error(
            "Cannot count scores in database",
        )),
    }
}

pub fn count_users(game_id: Option<Uuid>, pool: &Pool) -> Result<i64, ErrorResponse> {
    let mut game: Option<Game> = None;
    if let Some(id) = game_id {
        let fetched_game = game_service::find_by_id(id, pool);
        if fetched_game.is_err() {
            return Err(ResponseBody::not_found_error(&format!(
                "Game with id '{}' not found",
                id.to_string()
            )));
        }

        game = Some(fetched_game?)
    }

    match User::count(&game, &mut pool.get().unwrap()) {
        Ok(count) => Ok(count),
        Err(_) => Err(ResponseBody::internal_error(
            "Cannot count users in database",
        )),
    }
}
