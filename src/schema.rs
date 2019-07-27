table! {
    people (id) {
        id -> Int4,
        first_name -> Varchar,
        last_name -> Varchar,
        age -> Int4,
        profession -> Varchar,
        salary -> Int4,
    }
}

table! {
    players (id) {
        id -> Int4,
        steam_id -> Varchar,
        steam_name -> Varchar,
        num_games -> Int4,
        elo -> Int4,
    }
}

allow_tables_to_appear_in_same_query!(
    people,
    players,
);
