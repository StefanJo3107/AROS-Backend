use std::sync::Arc;

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
    fn routes(app: &mut Fianchetto, conn_pool: Arc<Pool<ConnectionManager<PgConnection>>>) {
        let conn = Arc::clone(&conn_pool);
        app.get("/turnir", move |_, _| {
            let turniri: Vec<Turnir>;
            match turnir::dsl::turnir.load(&conn.get().unwrap()) {
                Ok(t) => turniri = t,
                Err(err) => {
                    let err = err.to_string();
                    let err_json = json!({ "err": err });
                    return Ok(Response::not_found(serde_json::to_string(&err_json)?));
                }
            }

            let turniri_json = serde_json::to_string(&turniri)?;
            Ok(Response::ok(turniri_json))
        });

        let conn = Arc::clone(&conn_pool);
        app.get("/turnir/:id", move |_, params| {
            let turnir_id: i32 = params.find("id").unwrap().parse()?;
            let turnir: Turnir;
            match turnir::dsl::turnir
                .filter(turnir::dsl::turnir_id.eq(turnir_id))
                .first(&conn.get().unwrap())
            {
                Ok(t) => turnir = t,
                Err(err) => {
                    let err = err.to_string();
                    let err_json = json!({ "err": err });
                    return Ok(Response::not_found(serde_json::to_string(&err_json)?));
                }
            }

            let turnir_json = serde_json::to_string(&turnir)?;
            Ok(Response::ok(turnir_json))
        });

        let conn = Arc::clone(&conn_pool);
        app.post("/turnir", move |req, _| {
            let new_turnir: NewTurnir = serde_json::from_value(req.content)?;

            let turnir: Turnir = TurnirController::create_turnir(&conn.get().unwrap(), new_turnir)?;

            let turnir_json = serde_json::to_string(&turnir)?;
            Ok(Response::created(turnir_json))
        });

        let conn = Arc::clone(&conn_pool);
        app.delete("/turnir/:id", move |_, params| {
            let turnir_id: i32 = params.find("id").unwrap().parse()?;
            match TurnirController::delete_turnir(&conn.get().unwrap(), turnir_id) {
                Ok(()) => Ok(Response::ok(String::from(""))),
                Err(err) => {
                    return Ok(Response::bad_request_body(serde_json::to_string(
                        &json!({ "err": err.to_string() }),
                    )?))
                }
            }
        });

        let conn = Arc::clone(&conn_pool);
        app.put("/turnir/:id", move |req, params| {
            let turnir_id: i32 = params.find("id").unwrap().parse()?;
            let upd_turnir: NewTurnir = serde_json::from_value(req.content)?;

            let turnir: Turnir =
                TurnirController::update_turnir(&conn.get().unwrap(), turnir_id, upd_turnir)?;

            let turnir_json = serde_json::to_string(&turnir)?;
            Ok(Response::ok(turnir_json))
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
                dsl::turnir_slika.eq(turnir.turnir_slika),
            ))
            .get_result(conn)
    }
}
