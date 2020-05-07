#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;
extern crate reqwest;
extern crate serde_json;

mod store;
mod routes;

use crate::store::TinStore;

fn main() {

    let store = TinStore::new();

    rocket::ignite()
        .manage(store)
        .mount("/", routes![routes::get, routes::set, routes::set_exp, routes::delete])
        .launch();
}
