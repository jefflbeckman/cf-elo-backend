use connection::DbConn;
use diesel::result::Error;
use std::env;
use player;
use player::Player;
use rocket::http::Status;
use rocket::response::status;
use rocket_contrib::json::Json;

#[get("/")]
pub fn all(connection: DbConn) -> Result<Json<Vec<Player>>, Status> {
    player::repository::all(&connection)
        .map(|player| Json(player))
        .map_err(|error| error_status(error))
}

fn error_status(error: Error) -> Status {
    match error {
        Error::NotFound => Status::NotFound,
        _ => Status::InternalServerError
    }
}

#[get("/<id>")]
pub fn get(id: i32, connection: DbConn) -> Result<Json<Player>, Status> {
    player::repository::get(id, &connection)
        .map(|player| Json(player))
        .map_err(|error| error_status(error))
}

#[post("/", format = "application/json", data = "<player>")]
pub fn post(player: Json<Player>, connection: DbConn) -> Result<status::Created<Json<Player>>, Status> {
    player::repository::insert(player.into_inner(), &connection)
        .map(|player| player_created(player))
        .map_err(|error| error_status(error))
}

fn player_created(player: Player) -> status::Created<Json<Player>> {
    status::Created(
        format!("{host}:{port}/player/{id}", host = host(), port = port(), id = player.id).to_string(),
        Some(Json(player)))
}

fn host() -> String {
    env::var("ROCKET_ADDRESS").expect("ROCKET_ADDRESS must be set")
}

fn port() -> String {
    env::var("ROCKET_PORT").expect("ROCKET_PORT must be set")
}

#[put("/<id>", format = "application/json", data = "<player>")]
pub fn put(id: i32, player: Json<Player>, connection: DbConn) -> Result<Json<Player>, Status> {
    player::repository::update(id, player.into_inner(), &connection)
        .map(|player| Json(player))
        .map_err(|error| error_status(error))
}

#[delete("/<id>")]
pub fn delete(id: i32, connection: DbConn) -> Result<Status, Status> {
    match player::repository::get(id, &connection) {
        Ok(_) => player::repository::delete(id, &connection)
            .map(|_| Status::NoContent)
            .map_err(|error| error_status(error)),
        Err(error) => Err(error_status(error))
    }
}
