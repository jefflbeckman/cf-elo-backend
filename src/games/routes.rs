use connection::DbConn;
use diesel::result::Error;
use std::env;

use games;
use games::GameEndResponse
use games::GameEndPost
use rocket::http::Status;
use rocket::response::status;
use rocket_contrib::json::Json;


fn error_status(error: Error) -> Status {
    match error {
        Error::NotFound => Status::NotFound,
        _ => Status::InternalServerError
    }
}

#[post("/game")]
pub fn all(connection: DbConn) -> Result<Json<Vec<Player>>, Status> {
    games::repository::all(&connection)
        .map(|games| Json(games))
        .map_err(|error| error_status(error))
}


#[get("/game/<id>")]
pub fn get(id: i32, connection: DbConn) -> Result<Json<Player>, Status> {
    games::repository::get(id, &connection)
        .map(|games| Json(games))
        .map_err(|error| error_status(error))
}

#[post("/", format = "application/json", data = "<games>")]
pub fn post(games: Json<Player>, connection: DbConn) -> Result<status::Created<Json<GameEndResponse>>, Status> {
    games::repository::insert(games.into_inner(), &connection)
        .map(|games| games_created(games))
        .map_err(|error| error_status(error))
}

fn games_created(games: Player) -> status::Created<Json<Player>> {
    status::Created(
        format!("{host}:{port}/games/{id}", host = host(), port = port(), id = games.id).to_string(),
        Some(Json(games)))
}

fn host() -> String {
    env::var("ROCKET_ADDRESS").expect("ROCKET_ADDRESS must be set")
}

fn port() -> String {
    env::var("ROCKET_PORT").expect("ROCKET_PORT must be set")
}

#[put("/<id>", format = "application/json", data = "<games>")]
pub fn put(id: i32, games: Json<Player>, connection: DbConn) -> Result<Json<Player>, Status> {
    games::repository::update(id, games.into_inner(), &connection)
        .map(|games| Json(games))
        .map_err(|error| error_status(error))
}

#[delete("/<id>")]
pub fn delete(id: i32, connection: DbConn) -> Result<Status, Status> {
    match games::repository::get(id, &connection) {
        Ok(_) => games::repository::delete(id, &connection)
            .map(|_| Status::NoContent)
            .map_err(|error| error_status(error)),
        Err(error) => Err(error_status(error))
    }
}
