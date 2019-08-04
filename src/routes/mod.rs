#![allow(proc_macro_derive_resolution_fallback)]
use connection::DbConn;
use diesel::result::Error;
use diesel::insert_into;
use diesel::prelude::*;

use rocket::http::Status;
use rocket_contrib::json::Json;

extern crate serde_json;
use std::*;
use crate::schema::*;
use crate::dbmodels::players::*;
use crate::dbmodels::links::*;
use crate::dbmodels::games::*;
use crate::dbmodels::races::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct GameEndPost {
    pub good_guys: Vec<String>,
    pub good_guys_races: Vec<String>,
    pub bad_guys: Vec<String>,
    pub bad_guys_races: Vec<String>,
    pub good_guys_won: bool
}

#[derive(Serialize, Deserialize)]
pub struct GameEndPostResponse {
    pub msg: String
}

fn handle_new_player(id: &String, connection: &DbConn) -> Result<PlayerRow, Status> {
    let player = players::table.filter(players::steam_id.like(&id)).get_result::<PlayerRow>(&**connection);
    match player {
        Ok(p) => Ok(p),
        Err(Error::NotFound) => {
            let new_player_insert = PlayerRowInsert {
                steam_id: id.clone(),
                elo: 1200
            };
            insert_into(players::table).values(&new_player_insert).get_result(&**connection)
                .map_err( |e| {
                    println!("Unable to add player to database due to {}", e);
                    Status::InternalServerError
                })
        },
        Err(e) => {
            println!("{}",e);
            Err(Status::InternalServerError)
        }
    } 
}

fn handle_new_race(desc: &String, connection: &DbConn) -> Result<RaceRow, Status> {
    let race = races::table.filter(races::desc.like(&desc)).get_result(&**connection);
    match race {
        Ok(p) => Ok(p),
        Err(Error::NotFound) => {
            let new_race_insert = RaceRowInsert {
                desc: desc.clone()
            };
            insert_into(races::table).values(new_race_insert).get_result(&**connection)
                .map_err( |e| {
                    println!("Unable to add race to database due to {}", e);
                    Status::InternalServerError
            })
        }
        Err(e) => {
            println!("{}",e);
            Err(Status::InternalServerError)
        }
    }    
}

fn handle_new_game(good_guys_won: bool, connection: &DbConn) -> Result<GameRow, Status> {
    insert_into(games::table).values(GameRowInsert {
       upload_time: std::time::SystemTime::now(),
       map_version: String::from("ELO_BETA"),
       good_guys_won: good_guys_won
    }).get_result(&**connection)
        .map_err( |e| {
            println!("Unable to add race to database due to {}", e);
            Status::InternalServerError
    })
}

#[get("/player/<id>")]
pub fn player_get(id: String, connection: DbConn) -> Result<Json<PlayerRow>, Status> {
    handle_new_player(&id, &connection).map( |x| Json(x))
}

#[get("/player")]
pub fn player_all(connection: DbConn) -> Result<Json<Vec<PlayerRow>>, Status> {
    players::table.load(&*connection)
        .map_err(|_| Status::InternalServerError)
        .map(|x| Json(x))

}

#[post("/game", format = "application/json", data = "<game_data>")]
pub fn game_post(game_data: Json<GameEndPost>, connection: DbConn) -> Result<Json<GameEndPostResponse>, Status> {
    if Json(&game_data).good_guys_races.len() != Json(&game_data).good_guys.len() {
        return Err(Status::BadRequest); 
    }
    if Json(&game_data).bad_guys_races.len() != Json(&game_data).bad_guys.len() {
        return Err(Status::BadRequest); 
    }

    /* Populate the tables with any new values */
    let good_guy_ids : Vec<PlayerRow>= Json(&game_data).good_guys.iter().map(|steam_id| handle_new_player(&steam_id, &connection).unwrap()).collect();
    let good_guy_race_ids : Vec<RaceRow> = Json(&game_data).good_guys_races.iter().map(|desc| handle_new_race(&desc, &connection).unwrap()).collect();
    let bad_guy_ids : Vec<PlayerRow>= Json(&game_data).bad_guys.iter().map(|steam_id| handle_new_player(&steam_id, &connection).unwrap()).collect();
    let bad_guy_race_ids : Vec<RaceRow> = Json(&game_data).bad_guys_races.iter().map(|desc| handle_new_race(&desc, &connection).unwrap()).collect();
    let game_id_row = handle_new_game(Json(&game_data).good_guys_won, &connection).unwrap();

    /* Link up new rows */
    for (steam_id, race) in good_guy_ids.iter().zip(good_guy_race_ids.iter()) {
        insert_into(games_players_link::table).values(GamePlayerLinkRowInsert {
            game_id: game_id_row.id,
            player_id: steam_id.id,
            race_id: race.id,
            good_guys: true
         }).execute(&*connection).map_err(|_| Status::InternalServerError)?;
    }
    for (steam_id, race) in bad_guy_ids.iter().zip(bad_guy_race_ids.iter()) {
        insert_into(games_players_link::table).values(GamePlayerLinkRowInsert {
            game_id: game_id_row.id,
            player_id: steam_id.id,
            race_id: race.id,
            good_guys: false
         }).execute(&*connection).map_err(|_| Status::InternalServerError)?;
    }
    /* Return response */
    Ok(Json(GameEndPostResponse {
        msg: format!("Added a new game, id = {}", game_id_row.id).to_string()
    }))
}

#[get("/games")]
pub fn games_all(connection: DbConn) -> Result<Json<Vec<GameRow>>, Status> {
    games::table.load(&*connection)
        .map_err(|_| Status::InternalServerError)
        .map(|x| Json(x))
}

#[get("/links")]
pub fn links_all(connection: DbConn) -> Result<Json<Vec<GamePlayerLinkRow>>, Status> {
    games_players_link::table.load(&*connection)
        .map_err(|_| Status::InternalServerError)
        .map(|x| Json(x))
}

