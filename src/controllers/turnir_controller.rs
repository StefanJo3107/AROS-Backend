use crate::models::{NewTurnir, Turnir};

use super::Controller;

use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::result::Error;
use serde_json::json;

use super::super::fianchetto::response::Response;
use super::super::fianchetto::Fianchetto;
use super::super::schema::turnir;
pub struct TurnirController;

impl Controller for TurnirController {
    fn routes(app: &mut Fianchetto<Pool<ConnectionManager<PgConnection>>>) {
        app.get("/turnir", |_, _, conn_pool| {
            let turniri: Vec<Turnir>;
            match turnir::dsl::turnir.load(&conn_pool.unwrap().get().unwrap()) {
                Ok(t) => turniri = t,
                Err(err) => {
                    let err = err.to_string();
                    let err_json = json!({ "err": err });
                    return Response::not_found(serde_json::to_string(&err_json).unwrap());
                }
            }

            let turniri_json = serde_json::to_string(&turniri).unwrap();
            Response::ok(turniri_json)
        });

        app.get("/turnir/:id", |_, params, conn_pool| {
            let turnir_id: i32 = params.find("id").unwrap().parse().unwrap();
            let turnir: Turnir;
            match turnir::dsl::turnir
                .filter(turnir::dsl::turnir_id.eq(turnir_id))
                .first(&conn_pool.unwrap().get().unwrap())
            {
                Ok(t) => turnir = t,
                Err(err) => {
                    let err = err.to_string();
                    let err_json = json!({ "err": err });
                    return Response::not_found(serde_json::to_string(&err_json).unwrap());
                }
            }

            let turnir_json = serde_json::to_string(&turnir).unwrap();
            Response::ok(turnir_json)
        });

        app.post("/turnir", |req, _, conn_pool| {
            let new_turnir: NewTurnir;
            match serde_json::from_value(req.content) {
                Ok(n) => new_turnir = n,
                Err(err) => {
                    return Response::bad_request_body(
                        serde_json::to_string(&json!({ "err": err.to_string() })).unwrap(),
                    )
                }
            };

            let turnir: Turnir;
            match TurnirController::create_turnir(&conn_pool.unwrap().get().unwrap(), new_turnir) {
                Ok(t) => turnir = t,
                Err(err) => {
                    return Response::bad_request_body(
                        serde_json::to_string(&json!({ "err": err.to_string() })).unwrap(),
                    )
                }
            };

            let turnir_json = serde_json::to_string(&turnir).unwrap();
            Response::created(turnir_json)
        });
    }
}

impl TurnirController {
    fn create_turnir(conn: &PgConnection, turnir: NewTurnir) -> Result<Turnir, Error> {
        diesel::insert_into(turnir::table)
            .values(&turnir)
            .get_result(conn)
    }
}
