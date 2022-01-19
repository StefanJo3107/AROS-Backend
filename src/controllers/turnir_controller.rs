use crate::models::{NewTurnir, Turnir};

use super::Controller;

use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::result::Error;
use serde_json::json;

use crate::fianchetto::response::Response;
use crate::fianchetto::Fianchetto;
use crate::schema::turnir;
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

        app.delete("/turnir/:id", |_, params, conn_pool| {
            let turnir_id: i32 = params.find("id").unwrap().parse().unwrap();
            match TurnirController::delete_turnir(&conn_pool.unwrap().get().unwrap(), turnir_id) {
                Ok(()) => Response::ok(String::from("")),
                Err(err) => {
                    return Response::bad_request_body(
                        serde_json::to_string(&json!({ "err": err.to_string() })).unwrap(),
                    )
                }
            }
        });

        app.put("/turnir/:id", |req, params, conn_pool| {
            let turnir_id: i32 = params.find("id").unwrap().parse().unwrap();
            let upd_turnir: NewTurnir;
            match serde_json::from_value(req.content) {
                Ok(u) => upd_turnir = u,
                Err(err) => {
                    return Response::bad_request_body(
                        serde_json::to_string(&json!({ "err": err.to_string() })).unwrap(),
                    )
                }
            };

            let turnir: Turnir;

            match TurnirController::update_turnir(
                &conn_pool.unwrap().get().unwrap(),
                turnir_id,
                upd_turnir,
            ) {
                Ok(t) => turnir = t,
                Err(err) => {
                    return Response::bad_request_body(
                        serde_json::to_string(&json!({ "err": err.to_string() })).unwrap(),
                    )
                }
            }

            let turnir_json = serde_json::to_string(&turnir).unwrap();
            Response::ok(turnir_json)
        });
    }
}

impl TurnirController {
    fn create_turnir(conn: &PgConnection, turnir: NewTurnir) -> Result<Turnir, Error> {
        diesel::insert_into(turnir::table)
            .values(&turnir)
            .get_result(conn)
    }

    fn delete_turnir(conn: &PgConnection, id: i32) -> Result<(), Error> {
        diesel::delete(turnir::dsl::turnir.filter(turnir::dsl::turnir_id.eq(id))).execute(conn)?;
        Ok(())
    }

    fn update_turnir(conn: &PgConnection, id: i32, turnir: NewTurnir) -> Result<Turnir, Error> {
        use turnir::dsl;
        diesel::update(dsl::turnir.find(id))
            .set((
                dsl::turnir_naziv.eq(turnir.turnir_naziv),
                dsl::broj_rundi.eq(turnir.broj_rundi),
                dsl::lokacija_id.eq(turnir.lokacija_id),
                dsl::turnir_datum.eq(turnir.turnir_datum),
            ))
            .get_result(conn)
    }
}
