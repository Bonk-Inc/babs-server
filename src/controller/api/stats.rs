use axum::extract::{Path, State};
use uuid::Uuid;

use crate::{
    models::stats::{GameStats, GlobalStats},
    response::{ErrorResponse, ResponseBody},
    service::stats_service,
    SharedState,
};

pub async fn all(
    State(app_state): State<SharedState>,
) -> Result<ResponseBody<GlobalStats>, ErrorResponse> {
    let pool = &app_state.read().unwrap().db;
    let game_count = stats_service::count_games(&pool)?;
    let score_count = stats_service::count_scores(None, &pool)?;
    let user_count = stats_service::count_users(None, &pool)?;

    let stats = GlobalStats {
        games: game_count,
        scores: score_count,
        users: user_count,
    };

    Ok(ResponseBody::ok("Global stats fetched", stats))
}

pub async fn game_stats(
    Path(game_id): Path<Uuid>,
    State(app_state): State<SharedState>,
) -> Result<ResponseBody<GameStats>, ErrorResponse> {
    let pool = &app_state.read().unwrap().db;
    let score_count = stats_service::count_scores(Some(game_id), pool)?;
    let user_count = stats_service::count_users(Some(game_id), pool)?;

    let game_stats = GameStats {
        scores: score_count,
        users: user_count,
    };

    Ok(ResponseBody::ok("Game stats fetched", game_stats))
}
