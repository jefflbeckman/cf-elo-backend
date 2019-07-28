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

mod games;
#mod stats;
#mod racestats;
mod connection;

fn main() {
    dotenv().ok();
    rocket::ignite()
        .manage(connection::init_pool())
        .mount("/",routes![games::handler::post,
                            //stats::handler::all,
                            //stats::handler::get,
                            //stats::handler::post,
                            //racestats::handler::post,
                            //racestats::handler::all,
                            //racestats::handler::get
                         ])
        .launch();
}
