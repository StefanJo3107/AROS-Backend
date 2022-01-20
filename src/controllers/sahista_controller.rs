use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::result::Error;
use serde_json::json;

use super::Controller;
use crate::fianchetto::response::Response;
use crate::fianchetto::Fianchetto;
use crate::models::{NewSahista, Sahista};
use crate::schema::sahista;
use crate::schema::sahista::dsl;

pub struct SahistaController;

impl Controller for SahistaController {
    fn routes(app: &mut Fianchetto<Pool<ConnectionManager<PgConnection>>>) {
        app.get("/sahista", |_, _, conn_pool| {
            let sahisti: Vec<Sahista>;
            match dsl::sahista.load(&conn_pool.unwrap().get().unwrap()) {
                Ok(s) => sahisti = s,
                Err(err) => {
                    let err = err.to_string();
                    let err_json = json!({ "err": err });
                    return Ok(Response::not_found(serde_json::to_string(&err_json)?));
                }
            }

            let sahisti_json = serde_json::to_string(&sahisti)?;
            Ok(Response::ok(sahisti_json))
        });

        app.get("/sahista/:id", |_, params, conn_pool| {
            let sahista_id: i32 = params.find("id").unwrap().parse()?;
            let sahista: Sahista = dsl::sahista
                .filter(dsl::sahista_id.eq(sahista_id))
                .first(&conn_pool.unwrap().get().unwrap())?;

            let sahista_json = serde_json::to_string(&sahista)?;
            Ok(Response::ok(sahista_json))
        });

        app.post("/sahista", |req, _, conn_pool| {
            let new_sahista: NewSahista = serde_json::from_value(req.content)?;
            let sahista: Sahista =
                SahistaController::create_sahista(&conn_pool.unwrap().get().unwrap(), new_sahista)?;

            let sahista_json = serde_json::to_string(&sahista)?;
            Ok(Response::created(sahista_json))
        });

        app.put("/sahista/:id", |req, params, conn_pool| {
            let sahista_id: i32 = params.find("id").unwrap().parse().unwrap();
            let upd_sahista: NewSahista = serde_json::from_value(req.content)?;

            let sahista: Sahista = SahistaController::update_sahista(
                &conn_pool.unwrap().get().unwrap(),
                sahista_id,
                upd_sahista,
            )?;

            let sahista_json = serde_json::to_string(&sahista)?;
            Ok(Response::ok(sahista_json))
        });
    }
}

impl SahistaController {
    fn create_sahista(conn: &PgConnection, sahista: NewSahista) -> Result<Sahista, Error> {
        diesel::insert_into(sahista::table)
            .values(&sahista)
            .get_result(conn)
    }

    fn update_sahista(conn: &PgConnection, id: i32, sahista: NewSahista) -> Result<Sahista, Error> {
        diesel::update(dsl::sahista.find(id))
            .set((
                dsl::titula_fide.eq(sahista.titula_fide),
                dsl::elo.eq(sahista.elo),
                dsl::ime.eq(sahista.ime),
                dsl::prezime.eq(sahista.prezime),
                dsl::lokacija_id.eq(sahista.lokacija_id),
                dsl::sahista_slika.eq(sahista.sahista_slika),
            ))
            .get_result(conn)
    }
}
