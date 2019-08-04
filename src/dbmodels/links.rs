#![allow(proc_macro_derive_resolution_fallback)]
use crate::schema::games_players_link;

#[derive(Insertable)]
#[table_name="games_players_link"]
pub struct GamePlayerLinkRowInsert {
    pub game_id: i32, 
    pub player_id: i32, 
    pub race_id: i32
}
#[derive(Queryable, AsChangeset)]
#[table_name="games_players_link"]
pub struct GamePlayerLinkRow {
    pub id: i32,
    pub game_id: i32, 
    pub player_id: i32, 
    pub race_id: i32
}

impl GamePlayerLinkRowInsert {
    fn from_row(row: GamePlayerLinkRow) -> GamePlayerLinkRowInsert {
        GamePlayerLinkRowInsert {
            game_id: row.game_id, 
            player_id: row.player_id, 
            race_id: row.race_id
        }
    }
}
