#![allow(proc_macro_derive_resolution_fallback)]

use diesel;
use diesel::prelude::*;
use schema::players;
use player::Player;

pub fn all(connection: &PgConnection) -> QueryResult<Vec<Player>> {
    players::table.load::<Player>(&*connection)
}

pub fn get(id: i32, connection: &PgConnection) -> QueryResult<Player> {
    players::table.find(id).get_result::<Player>(connection)
}

pub fn insert(person: Player, connection: &PgConnection) -> QueryResult<Player> {
    diesel::insert_into(players::table)
        .values(&InsertablePlayer::from_player(person))
        .get_result(connection)
}

pub fn update(id: i32, person: Player, connection: &PgConnection) -> QueryResult<Player> {
    diesel::update(players::table.find(id))
        .set(&person)
        .get_result(connection)
}

pub fn delete(id: i32, connection: &PgConnection) -> QueryResult<usize> {
    diesel::delete(players::table.find(id))
        .execute(connection)
}

#[derive(Insertable)]
#[table_name="players"]
struct InsertablePlayer {
    steam_id: String,
    steam_name: String,
    num_games: i32,
}

impl InsertablePlayer {

    fn from_player(person: Player) -> InsertablePlayer {
        InsertablePlayer {
            steam_id: person.steam_id,
            steam_name: person.steam_name,
            num_games: person.num_games,
        }
    }
}
