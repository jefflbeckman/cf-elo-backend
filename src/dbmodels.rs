#![allow(proc_macro_derive_resolution_fallback)]
use super::schema::games;
use super::schema::games_players_link;
use chrono::NaiveDateTime
pub mod handler;
pub mod repository;

#[derive(Serialize, Deserialize)]
pub struct GameEndPost {
    pub num_players: i32,
    pub steam_ids: Vec<String>,
    pub steam_name: Vec<String>,
    pub leaves: Vec<i32>,
    pub races: Vec<String>
    pub num_games: i32,
    pub good_guys_won: bool
}

#[derive(Serialize, Deserialize)]
pub struct GameEndResponse {
    pub msg: String
}

#[derive(Insertable)]
#[table_name="games_players_link"]
pub struct GamePlayerLinkRowInsert {
    pub game_id: i32, 
    pub player_id: i32, 
    pub race_id: i32, 
    pub leaver: i32
}
#[derive(Queryable, AsChangeset)]
#[table_name="games_players_link"]
pub struct GamePlayerLinkRow {
    pub id: i32,
    pub game_id: i32, 
    pub player_id: i32, 
    pub race_id: i32, 
    pub leaver: i32
}

impl GamePlayerLinkRowInsert {
    fn from_row(row: GamePlayerLinkRow) -> GamePlayerLinkRowInsert {
        GamePlayerLinkRowInsert {
            game_id: row.game_id, 
            player_id: row.player_id, 
            race_id: row.race_id, 
            leaver: row.leaver, 
        }
    }
}

#[derive(Queryable, AsChangeset)]
#[table_name="games"]
pub struct GameRow {
    pub id: i32,
    pub map_version: String,
    pub winner: bool
}

#[derive(Insertable)]
#[table_name="games"]
pub struct GameRowInsert {
    pub map_version: String,
    pub winner: bool
}

impl GameRowInsert {
    fn from_row(row: GameRow) -> GameRowInsert {
        GameRowInsert {
            map_verion: row.map_version, 
            good_guys_won: row.good_guys_won
        }
    }
}

#[derive(Queryable, AsChangeset)]
#[table_name="races"]
pub struct RaceRow {
    pub id: i32,
    pub desc: String
}

#[derive(Insertable)]
#[table_name="races"]
pub struct RaceRowInsert {
    pub desc: String
}

impl RaceRowInsert {
    fn from_row(row: RaceRow) -> RaceRowInsert {
        RaceRowInsert {
            desc: row.desc
        }
    }
}

pub mod handler;
pub mod repository;

#[derive(Insertable)]
#[table_name="players"]
pub struct PlayerRowInsert {
    pub steam_id: String,
    pub elo: i32
}

#[derive(Queryable, AsChangeset)]
#[table_name="players"]
pub struct PlayerRow {
    pub id: i32,
    pub steam_id: String,
    pub elo: i32
}

impl PlayerRowInsert {
    fn from_row(row: PlayerRow) -> PlayerRowInsert {
        PlayerRowInsert {
            steam_id: row.steam_id,
            elo: row.elo
        }
    }
}
