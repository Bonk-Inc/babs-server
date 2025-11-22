use chrono::NaiveDateTime;
use diesel::{dsl::count_star, prelude::*, result::Error, AsChangeset, Insertable};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::{
    config::db::Connection,
    models::{game::Game, level::Level, user::User},
    schema::{level, score, user},
};

#[derive(Associations, Identifiable, Queryable, Selectable)]
#[diesel(table_name = score)]
#[diesel(belongs_to(Level))]
#[diesel(belongs_to(User))]
pub struct Score {
    pub id: Uuid,
    pub username: Option<String>,
    pub highscore: i32,
    pub is_hidden: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    pub level_id: Option<Uuid>,
    pub user_id: Option<Uuid>,
}

#[derive(Insertable, AsChangeset, Deserialize, ToSchema, Clone)]
#[diesel(table_name = score)]
pub struct ScoreForm {
    pub username: Option<String>,
    #[serde(rename = "score")]
    pub highscore: i32,
    pub is_hidden: bool,
    pub level_id: Uuid,
    pub user_id: Option<Uuid>,
}

#[derive(Serialize, ToSchema)]
pub struct ScoreDto {
    pub id: Uuid,
    pub score: i32,
    pub is_hidden: bool,
    pub username: Option<String>,
    pub level: Option<Level>,
    pub user: Option<User>,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}

impl Into<ScoreDto> for (Score, Option<Level>, Option<User>) {
    fn into(self) -> ScoreDto {
        let (score, level, user) = self;

        ScoreDto {
            id: score.id,
            score: score.highscore,
            is_hidden: score.is_hidden,
            username: score.username,
            user,
            level,
            created_at: score.created_at,
            updated_at: score.updated_at,
        }
    }
}

impl Score {
    /// Fetches all the scores in the database by looking up all the levels related to the given game. And using
    /// the levels to fetch all the scores.
    pub fn find_all(game: &Game, conn: &mut Connection) -> Result<Vec<ScoreDto>, Error> {
        let levels = Level::find_by_game(game, conn)?;
        let scores = Score::belonging_to(&levels)
            .left_join(user::table)
            .left_join(level::table)
            .select((Score::as_select(), Option::<Level>::as_select(), Option::<User>::as_select()))
            .load::<(Score, Option<Level>, Option<User>)>(conn)?
            .into_iter()
            .map(|item| item.into())
            .collect::<Vec<ScoreDto>>();

        Ok(scores)
    }

    /// Fetches a score from the database with the given id.
    /// 
    /// # Errors
    /// - If no score is found with the given id.
    pub fn find_by_id(score_id: Uuid, conn: &mut Connection) -> Result<ScoreDto, Error> {
        let result = score::dsl::score.find(score_id)
            .left_join(user::table)
            .left_join(level::table)
            .select((Score::as_select(), Option::<Level>::as_select(), Option::<User>::as_select()))
            .get_result::<(Score, Option<Level>, Option<User>)>(conn)?;

        Ok(result.into())
    }

    /// Fetches all the scores in the database related to the given level.
    pub fn find_by_level(
        level: &Level,
        include_hidden: bool,
        conn: &mut Connection,
    ) -> Result<Vec<ScoreDto>, Error> {
        let mut query = Score::belonging_to(level)
            .into_boxed();

        if !include_hidden {
            query = query.filter(score::dsl::is_hidden.eq(false));
        }

        let scores = query
            .left_join(user::table)
            .left_join(level::table)
            .select((Score::as_select(), Option::<Level>::as_select(), Option::<User>::as_select()))
            .load::<(Score, Option<Level>, Option<User>)>(conn)?
            .into_iter()
            .map(|item| item.into())
            .collect::<Vec<ScoreDto>>();

        Ok(scores)
    }

    /// Fetches all the scores in the database related to the given user.
    pub fn find_by_user(
        user: &User,
        include_hidden: bool,
        conn: &mut Connection,
    ) -> Result<Vec<ScoreDto>, Error> {
        let mut query = Score::belonging_to(user)
            .into_boxed();

        if !include_hidden {
            query = query.filter(score::dsl::is_hidden.eq(false));
        }

        let scores = query
            .left_join(user::table)
            .left_join(level::table)
            .select((Score::as_select(), Option::<Level>::as_select(), Option::<User>::as_select()))
            .load::<(Score, Option<Level>, Option<User>)>(conn)?
            .into_iter()
            .map(|item| item.into())
            .collect::<Vec<ScoreDto>>();

        Ok(scores)
    }

    /// Adds a new score to the database.
    /// 
    /// Errors
    /// - If one of the fields contain invalid data.
    pub fn insert(new_score: ScoreForm, conn: &mut Connection) -> Result<ScoreDto, Error> {
        let inserted_score = diesel::insert_into(score::dsl::score)
            .values(&new_score)
            .get_result::<Score>(conn)?;

        let level = Level::find_by_id(new_score.level_id, conn)?;
        let user = if let Some(user_id) = new_score.user_id {
            Some(User::find_by_id(user_id, conn)?)
        } else { None };

        Ok((inserted_score, Some(level), user).into())
    }

    /// Updates a score with the given id in the database.
    /// 
    /// Errors
    /// - If no score is found with the given id.
    /// - If one of the fields contain invalid data.
    pub fn update(
        score_id: Uuid,
        updated_score: ScoreForm,
        conn: &mut Connection,
    ) -> Result<ScoreDto, Error> {
        let score = diesel::update(score::dsl::score)
            .filter(score::dsl::id.eq(score_id))
            .set(updated_score.clone())
            .get_result::<Score>(conn)?;

        let level = Level::find_by_id(updated_score.level_id, conn)?;
        let user = if let Some(user_id) = updated_score.user_id {
            Some(User::find_by_id(user_id, conn)?)
        } else { None };

        Ok((score, Some(level), user).into())
    }

    /// Deletes multiple scores from the database with the given ids.
    pub fn delete_many(score_ids: Vec<Uuid>, conn: &mut Connection) -> QueryResult<usize> {
        diesel::delete(score::dsl::score)
            .filter(score::dsl::id.eq_any(score_ids))
            .execute(conn)
    }

    /// Counts the number of scores in the database. If a game is given, only the number of scores relates to the game are
    /// counted.
    pub fn count(game: &Option<Game>, conn: &mut Connection) -> Result<i64, Error> {
        let count = if let Some(value) = game {
            let levels = Level::belonging_to(value)
                .select(Level::as_select())
                .load(conn)?;

            Score::belonging_to(&levels)
                .select(count_star())
                .first(conn)?
        } else {
            score::dsl::score.select(count_star()).first(conn)?
        };

        Ok(count)
    }
}