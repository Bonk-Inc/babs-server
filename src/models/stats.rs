use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, Default, ToSchema)]
pub struct GlobalStatsDto {
    pub games: i64,
    pub scores: i64,
    pub users: i64
}

#[derive(Serialize, Default, ToSchema)]
pub struct GameStatsDto {
    pub scores: i64,
    pub users: i64
}