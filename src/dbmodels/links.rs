#![allow(proc_macro_derive_resolution_fallback)]
use crate::schema::games_players_link;

#[derive(Insertable)]
#[table_name="games_players_link"]
pub struct GamePlayerLinkRowInsert {
    pub game_id: i32, 
    pub player_id: i32, 
    pub race_id: i32,
    pub good_guys: bool
}
#[derive(Queryable, AsChangeset, Serialize, Deserialize)]
#[table_name="games_players_link"]
pub struct GamePlayerLinkRow {
    pub id: i32,
    pub game_id: i32, 
    pub player_id: i32, 
    pub race_id: i32,
    pub good_guys: bool
}
