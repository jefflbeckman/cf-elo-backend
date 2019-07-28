table! {
    games (id) {
        id -> Int4,
        upload_time -> Nullable<Timestamp>,
        map_version -> Nullable<Varchar>,
        winner -> Nullable<Bool>,
    }
}

table! {
    games_players_link (id) {
        id -> Int4,
        game_id -> Nullable<Int4>,
        player_id -> Nullable<Int4>,
        race_id -> Nullable<Int4>,
        leaver -> Nullable<Int4>,
    }
}

table! {
    players (id) {
        id -> Int4,
        steam_id -> Varchar,
        elo -> Int4,
    }
}

table! {
    races (id) {
        id -> Int4,
        desc -> Nullable<Varchar>,
    }
}

joinable!(games_players_link -> games (game_id));
joinable!(games_players_link -> players (player_id));
joinable!(games_players_link -> races (race_id));

allow_tables_to_appear_in_same_query!(
    games,
    games_players_link,
    players,
    races,
);
