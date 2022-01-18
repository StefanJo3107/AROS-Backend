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
    fn routes(app: &mut Fianchetto<Pool<ConnectionManager<PgConnection>>>) {
        app.get("/lokacija", |_, _, conn_pool| {
            let results: Vec<Lokacija>;
            match LokacijaController::get_lokacija(&conn_pool.unwrap().get().unwrap()) {
                Ok(res) => results = res,
                Err(err) => {
                    let err = err.to_string();
                    let err_json = json!({ "err": err });
                    return Response::not_found(serde_json::to_string(&err_json).unwrap());
                }
            }

            let lok_json = serde_json::to_string(&results).unwrap();
            Response::ok(lok_json)
        });

        app.get("/lokacija/:id", |_, params, conn_pool| {
            let lok_id: i32 = params.find("id").unwrap().parse().unwrap();
            let result: Lokacija;
            match lokacija::dsl::lokacija
                .filter(lokacija::dsl::id.eq(lok_id))
                .first::<Lokacija>(&conn_pool.unwrap().get().unwrap())
            {
                Ok(res) => result = res,
                Err(err) => {
                    let err = err.to_string();
                    let err_json = json!({ "err": err });
                    return Response::not_found(serde_json::to_string(&err_json).unwrap());
                }
            };

            let lok_json = serde_json::to_string(&result).unwrap();
            Response::ok(lok_json)
        });

        app.post("/lokacija", |req, _, conn_pool| {
            let lok_naziv;
            match req.content["naziv"].as_str() {
                Some(n) => lok_naziv = n,
                None => {
                    return Response::bad_request_body(
                        serde_json::to_string(&json!({"err": "Incorrect body format"})).unwrap(),
                    )
                }
            };
            let lok =
                LokacijaController::create_lokacija(&conn_pool.unwrap().get().unwrap(), lok_naziv);

            let lok_json = serde_json::to_string(&lok).unwrap();
            Response::created(lok_json)
        });
    }
}

impl LokacijaController {
    pub fn get_lokacija(conn: &PgConnection) -> QueryResult<Vec<Lokacija>> {
        lokacija::dsl::lokacija.load(conn)
    }

    fn create_lokacija(conn: &PgConnection, naziv: &str) -> Lokacija {
        let new_lokacija = NewLokacija { naziv };

        diesel::insert_into(lokacija::table)
            .values(&new_lokacija)
            .get_result(conn)
            .expect("Error saving new lokacija")
    }
}
