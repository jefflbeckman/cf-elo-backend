table! {
    games (id) {
        id -> Int4,
        upload_time -> Timestamp,
        map_version -> Varchar,
        good_guys_won -> Bool,
    }
}

table! {
    games_players_link (id) {
        id -> Int4,
        game_id -> Int4,
        player_id -> Int4,
        race_id -> Int4,
        good_guys -> Bool,
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
        desc -> Varchar,
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
