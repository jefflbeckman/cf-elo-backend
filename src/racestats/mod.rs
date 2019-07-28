#![allow(proc_macro_derive_resolution_fallback)]
use super::schema::players;

pub mod handler;
pub mod repository;

#[derive(Queryable, AsChangeset, Serialize, Deserialize)]
pub struct Player {
    pub id: i32,
    pub steam_id: String,
    pub steam_name: String,
    pub num_games: i32,
    pub elo: i32
}
