use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Default, ToSchema)]
pub struct GlobalStats {
    pub games: i64,
    pub scores: i64,
    pub users: i64
}

#[derive(Serialize, Deserialize, Default, ToSchema)]
pub struct GameStats {
    pub scores: i64,
    pub users: i64
}