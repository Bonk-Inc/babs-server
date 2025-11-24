use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::{
    config::db::Connection,
    models::game::Game,
    schema::level::{self, dsl::*},
};

#[derive(Serialize, Associations, Identifiable, Queryable, Selectable)]
#[diesel(table_name = level)]
#[diesel(belongs_to(Game))]
pub struct Level {
    pub id: Uuid,
    pub name: String,
    pub game_id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Serialize, ToSchema)]
pub struct LevelDto {
    pub id: Uuid,
    pub name: String,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Insertable, AsChangeset, Serialize, Deserialize, ToSchema)]
#[diesel(table_name = level)]
pub struct LevelFormDto {
    pub name: String,
    pub game_id: Uuid,
}

impl Into<LevelDto> for Level {
    fn into(self) -> LevelDto {
        LevelDto {
            id: self.id,
            name: self.name,
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}

impl Level {
    /// Fetches all the levels in the database.
    pub fn find_all(conn: &mut Connection) -> QueryResult<Vec<Level>> {
        level.load::<Level>(conn)
    }

    /// Fetches a level from the database with the given id.
    /// 
    /// # Errors
    /// - If no level is found with the given id.
    pub fn find_by_id(level_id: Uuid, conn: &mut Connection) -> QueryResult<Level> { 
        level.find(level_id)
            .get_result::<Level>(conn)
    }

    /// Fetches levels related to the given game from the database.
    pub fn find_by_game(game: &Game, conn: &mut Connection) -> QueryResult<Vec<Level>> {
        Level::belonging_to(game)
            .load::<Level>(conn)
    }    

    /// Adds a new level to the database.
    /// 
    /// Errors
    /// - If one of the fields contain invalid data.
    pub fn insert(data: LevelFormDto, conn: &mut Connection) -> QueryResult<Level> {
        diesel::insert_into(level)
            .values(&data)
            .get_result::<Level>(conn)
    }

    /// Updates a level with the given id in the database.
    /// 
    /// Errors
    /// - If no level is found with the given id.
    /// - If one of the fields contain invalid data.
    pub fn update(level_id: Uuid, data: LevelFormDto, conn: &mut Connection) -> QueryResult<Level> {
        diesel::update(level)
            .filter(id.eq(level_id))
            .set(data)
            .get_result::<Level>(conn)
    }

    /// Deletes a level with the given id from the database.
    pub fn delete(level_id: Uuid, conn: &mut Connection) -> QueryResult<usize> {
        diesel::delete(level).filter(id.eq(level_id)).execute(conn)
    }
}