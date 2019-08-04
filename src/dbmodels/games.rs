#![allow(proc_macro_derive_resolution_fallback)]
use crate::schema::games;

#[derive(Queryable, AsChangeset)]
#[table_name="games"]
pub struct GameRow {
    pub id: i32,
    pub upload_time: std::time::SystemTime,
    pub map_version: String,
    pub good_guys_won: bool
}

#[derive(Insertable)]
#[table_name="games"]
pub struct GameRowInsert {
    pub upload_time: std::time::SystemTime,
    pub map_version: String,
    pub good_guys_won: bool
}
