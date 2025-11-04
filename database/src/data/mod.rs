use rocket::fairing::AdHoc;
use rocket::{Build, Rocket};
use rocket_sync_db_pools::database;
use rocket_sync_db_pools::diesel;

pub mod farm;
pub mod user;
pub mod location;

