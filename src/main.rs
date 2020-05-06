#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;
extern crate reqwest;
extern crate serde_json;

mod store;
mod routes;

use crate::store::TinStore;
use clokwerk::{Scheduler, TimeUnits};
use clokwerk::Interval::*;
use std::time::Duration;

fn check_expired() {
    let _res = reqwest::get("http://localhost:8000/check");
}

fn main() {

    let store = TinStore::new();
    
    let mut scheduler = Scheduler::new();
    scheduler.every(5.seconds()).run(|| check_expired());

    let _thread_handle = scheduler.watch_thread(Duration::from_millis(100));

    rocket::ignite()
        .manage(store)
        .mount("/", routes![routes::get, routes::set, routes::set_exp, routes::delete, routes::check])
        .launch();
}
