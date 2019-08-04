#![allow(proc_macro_derive_resolution_fallback)]
use crate::schema::races;

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
