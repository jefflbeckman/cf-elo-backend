#![allow(proc_macro_derive_resolution_fallback)]
use crate::schema::players;

#[derive(Insertable)]
#[table_name="players"]
pub struct PlayerRowInsert {
    pub steam_id: String,
    pub elo: i32
}

#[derive(Queryable, AsChangeset, Serialize, Deserialize)]
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
