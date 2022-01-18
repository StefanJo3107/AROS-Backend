#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod concurrency;
pub mod controllers;
pub mod fianchetto;
pub mod models;
pub mod schema;

use diesel::pg::PgConnection;
use diesel::r2d2::*;
use dotenv::dotenv;
use models::Lokacija;
use std::env;

pub fn establish_connection() -> Pool<ConnectionManager<PgConnection>> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::new(database_url);
    Pool::builder().build(manager).unwrap()
}
