#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod models;
pub mod schema;

use diesel::pg::PgConnection;
use diesel::r2d2::*;
use diesel::RunQueryDsl;
use dotenv::dotenv;
use models::{Lokacija, NewLokacija};
use schema::lokacija;
use std::env;

pub fn establish_connection() -> Pool<ConnectionManager<PgConnection>> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::new(database_url);
    Pool::builder().build(manager).unwrap()
}

pub fn create_lokacija(conn: &PgConnection, naziv: &str) -> Lokacija {
    let new_lokacija = NewLokacija { naziv };

    diesel::insert_into(lokacija::table)
        .values(&new_lokacija)
        .get_result(conn)
        .expect("Error saving new lokacija")
}
