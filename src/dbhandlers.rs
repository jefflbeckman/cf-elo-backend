#![allow(proc_macro_derive_resolution_fallback)]

use diesel;
use diesel::prelude::*;
use schema::players;
use player::PlayerRow;
use player::PlayerRowInsert;

pub fn all(connection: &PgConnection) -> QueryResult<Vec<PlayerRow>> {
    players::table.load::<PlayerRow>(&*connection)
}

pub fn get(id: i32, connection: &PgConnection) -> QueryResult<PlayerRow> {
    players::table.find(id).get_result::<PlayerRow>(connection)
}

pub fn insert(row: PlayerRow, connection: &PgConnection) -> QueryResult<PlayerRow> {
    diesel::insert_into(players::table)
        .values(&PlayerRowInsert::from_row(row))
        .get_result(connection)
}

pub fn update(id: i32, row: PlayerRow, connection: &PgConnection) -> QueryResult<PlayerRow> {
    diesel::update(players::table.find(id))
        .set(&row)
        .get_result(connection)
}

pub fn delete(id: i32, connection: &PgConnection) -> QueryResult<usize> {
    diesel::delete(players::table.find(id))
        .execute(connection)
}
