use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::result::Error;
use serde_json::json;

use super::Controller;
use crate::fianchetto::response::Response;
use crate::fianchetto::Fianchetto;
use crate::models::{NewPartija, Partija};
use crate::schema::partija;
use crate::schema::partija::dsl;

pub struct PartijaController;

impl Controller for PartijaController {
    fn routes(app: &mut Fianchetto<Pool<ConnectionManager<PgConnection>>>) {
        app.get("/partija", |_, _, conn_pool| {
            let partije: Vec<Partija>;
            match dsl::partija.load(&conn_pool.unwrap().get().unwrap()) {
                Ok(p) => partije = p,
                Err(err) => {
                    let err = err.to_string();
                    let err_json = json!({ "err": err });
                    return Ok(Response::not_found(serde_json::to_string(&err_json)?));
                }
            }

            let partije_json = serde_json::to_string(&partije)?;
            Ok(Response::ok(partije_json))
        });

        app.get("/partija/:id", |_, params, conn_pool| {
            let partija_id: i32 = params.find("id").unwrap().parse()?;
            let partija: Partija;
            match dsl::partija
                .filter(dsl::partija_id.eq(partija_id))
                .first(&conn_pool.unwrap().get().unwrap())
            {
                Ok(p) => partija = p,
                Err(err) => {
                    let err = err.to_string();
                    let err_json = json!({ "err": err });
                    return Ok(Response::not_found(serde_json::to_string(&err_json)?));
                }
            }

            let partija_json = serde_json::to_string(&partija)?;
            Ok(Response::ok(partija_json))
        });

        app.post("/partija", |req, _, conn_pool| {
            let new_partija: NewPartija = serde_json::from_value(req.content)?;
            let partija: Partija =
                PartijaController::create_partija(&conn_pool.unwrap().get().unwrap(), new_partija)?;

            let partija_json = serde_json::to_string(&partija)?;
            Ok(Response::created(partija_json))
        });

        app.put("/partija/:id", |req, params, conn_pool| {
            let partija_id: i32 = params.find("id").unwrap().parse()?;
            let upd_partija: NewPartija = serde_json::from_value(req.content)?;
            let partija: Partija = PartijaController::update_partija(
                &conn_pool.unwrap().get().unwrap(),
                partija_id,
                upd_partija,
            )?;

            let partija_json = serde_json::to_string(&partija)?;
            Ok(Response::ok(partija_json))
        });
    }
}

impl PartijaController {
    fn create_partija(conn: &PgConnection, partija: NewPartija) -> Result<Partija, Error> {
        diesel::insert_into(partija::table)
            .values(&partija)
            .get_result(conn)
    }

    fn update_partija(conn: &PgConnection, id: i32, partija: NewPartija) -> Result<Partija, Error> {
        diesel::update(dsl::partija.find(id))
            .set((
                dsl::beli_id.eq(partija.beli_id),
                dsl::crni_id.eq(partija.crni_id),
                dsl::datum.eq(partija.datum),
                dsl::otvaranje.eq(partija.otvaranje),
                dsl::pgn.eq(partija.pgn),
                dsl::rezultat.eq(partija.rezultat),
                dsl::runda.eq(partija.runda),
                dsl::turnir_id.eq(partija.turnir_id),
            ))
            .get_result(conn)
    }
}
