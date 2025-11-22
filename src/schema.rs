// @generated automatically by Diesel CLI.

diesel::table! {
    game (id) {
        id -> Uuid,
        #[max_length = 50]
        name -> Varchar,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    level (id) {
        id -> Uuid,
        #[max_length = 50]
        name -> Varchar,
        game_id -> Uuid,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    score (id) {
        id -> Uuid,
        #[max_length = 50]
        username -> Nullable<Varchar>,
        #[sql_name = "score"]
        highscore -> Int4,
        is_hidden -> Bool,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
        level_id -> Nullable<Uuid>,
        user_id -> Nullable<Uuid>,
    }
}

diesel::table! {
    user (id) {
        id -> Uuid,
        #[max_length = 50]
        name -> Varchar,
        game_id -> Uuid,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::joinable!(level -> game (game_id));
diesel::joinable!(score -> level (level_id));
diesel::joinable!(score -> user (user_id));
diesel::joinable!(user -> game (game_id));

diesel::allow_tables_to_appear_in_same_query!(
    game,
    level,
    score,
    user,
);
