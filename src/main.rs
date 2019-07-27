#![feature(decl_macro, proc_macro_hygiene)]
#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;
extern crate dotenv;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;

use dotenv::dotenv;

mod people;
mod player;
mod schema;
mod connection;

fn main() {
    dotenv().ok();
    rocket::ignite().
        manage(connection::init_pool()).
        mount("/player",routes![player::handler::all,
                                player::handler::get,
                                player::handler::post,
                                player::handler::put,
                                player::handler::delete]).
        mount("/person",routes![people::handler::all,
                                people::handler::get,
                                people::handler::post,
                                people::handler::put,
                                people::handler::delete]).
    launch();
}
