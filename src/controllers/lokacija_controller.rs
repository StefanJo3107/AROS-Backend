use std::sync::Arc;
use std::thread;
use std::time::Duration;

use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use serde_json::json;

use crate::models::NewLokacija;

use super::super::fianchetto::response::Response;
use super::super::fianchetto::Fianchetto;
use super::super::schema::lokacija;
use super::super::Lokacija;
use super::Controller;
pub struct LokacijaController;

impl Controller for LokacijaController {
    fn routes(app: &mut Fianchetto, conn_pool: Arc<Pool<ConnectionManager<PgConnection>>>) {
        app.get("/sleep", |_, _| {
            thread::sleep(Duration::from_secs(5));
            Ok(Response::ok(String::from("Izvršen sleep")))
        });

        app.get("/nosleep", |_, _| {
            Ok(Response::ok(String::from("Izvršen nosleep")))
        });

        let conn = Arc::clone(&conn_pool);
        app.get("/lokacija", move |_, _| {
            let results: Vec<Lokacija>;
            match LokacijaController::get_lokacija(&conn.get().unwrap()) {
                Ok(res) => results = res,
                Err(err) => {
                    let err = err.to_string();
                    let err_json = json!({ "err": err });
                    return Ok(Response::not_found(serde_json::to_string(&err_json)?));
                }
            }

            let lok_json = serde_json::to_string(&results)?;
            Ok(Response::ok(lok_json))
        });

        let conn = Arc::clone(&conn_pool);
        app.get("/lokacija/:id", move |_, params| {
            let lok_id: i32 = params.find("id").unwrap().parse()?;
            let result: Lokacija;
            match lokacija::dsl::lokacija
                .filter(lokacija::dsl::id.eq(lok_id))
                .first::<Lokacija>(&conn.get().unwrap())
            {
                Ok(res) => result = res,
                Err(err) => {
                    let err = err.to_string();
                    let err_json = json!({ "err": err });
                    return Ok(Response::not_found(serde_json::to_string(&err_json)?));
                }
            };

            let lok_json = serde_json::to_string(&result)?;
            Ok(Response::ok(lok_json))
        });

        let conn = Arc::clone(&conn_pool);
        app.post("/lokacija", move |req, _| {
            let lok_naziv;
            match req.content["naziv"].as_str() {
                Some(n) => lok_naziv = n,
                None => {
                    return Ok(Response::bad_request_body(serde_json::to_string(
                        &json!({"err": "Incorrect body format"}),
                    )?))
                }
            };
            let lok = LokacijaController::create_lokacija(&conn.get().unwrap(), lok_naziv)?;

            let lok_json = serde_json::to_string(&lok)?;
            Ok(Response::created(lok_json))
        });
    }
}

impl LokacijaController {
    pub fn get_lokacija(conn: &PgConnection) -> QueryResult<Vec<Lokacija>> {
        lokacija::dsl::lokacija.load(conn)
    }

    fn create_lokacija(
        conn: &PgConnection,
        naziv: &str,
    ) -> Result<Lokacija, diesel::result::Error> {
        let new_lokacija = NewLokacija { naziv };

        diesel::insert_into(lokacija::table)
            .values(&new_lokacija)
            .get_result(conn)
    }
}
