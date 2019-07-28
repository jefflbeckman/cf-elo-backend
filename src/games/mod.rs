#![allow(proc_macro_derive_resolution_fallback)]
use super::schema::gamess;

pub mod handler;
pub mod repository;

#[derive(Queryable, AsChangeset, Serialize, Deserialize)]
// list of players
// when they left
// what race they played
// who won
// 

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

#[derive(Insertable)]
#[table_name="games"]
pub struct GamePlayerLinkRowInsert {
    pub game_id: i32, 
    pub player_id: i32, 
    pub race_id: i32, 
    pub leaver: i32
}
#[derive(Queryable, AsChangeset)]
#[table_name="games"]
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

#[derive(Serialize, Deserialize)]
pub struct GameEndResponse {
    pub msg: String
}
