#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;
extern crate short_crypt;
extern crate serde_json;

mod manager;
mod queue;
mod store;
mod routes;

use clap::{App, load_yaml};
use crate::store::TinStore;
use crate::manager::TinQueueManager;

fn main() {

    let yaml = load_yaml!("cli.yml");
    let app_matches = App::from(yaml).get_matches();

    let secret_key = app_matches.value_of("key").unwrap_or("");

    let store = TinStore::new(secret_key.to_string());
    let queue_manager = TinQueueManager::new();

    rocket::ignite()
        .manage(store)
        .manage(queue_manager)
        .mount("/", routes![routes::home, 
                            routes::get, 
                            routes::set, 
                            routes::set_exp, 
                            routes::delete, 
                            routes::get_queue, 
                            routes::create_queue, 
                            routes::delete_queue,
                            routes::push_to_queue,
                            routes::clear_queue])
        .launch();
}
