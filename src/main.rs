mod concurrency;
mod fianchetto;

use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;
use fianchetto::response::Response;
use fianchetto::Fianchetto;
use micro_backend_framework::create_lokacija;
use micro_backend_framework::establish_connection;
use micro_backend_framework::models::*;
use micro_backend_framework::schema::lokacija::dsl::*;
use serde_json::json;

fn main() {
    let mut app: Fianchetto<Pool<ConnectionManager<PgConnection>>> =
        Fianchetto::new("127.0.0.1:1207", 4);
    app.set_db_connection(establish_connection());

    app.get("/lokacija", |_, _, conn_pool| {
        let results: Vec<Lokacija>;
        match lokacija.load(&conn_pool.unwrap().get().unwrap()) {
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
        match lokacija
            .filter(id.eq(lok_id))
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
        let lok = create_lokacija(&conn_pool.unwrap().get().unwrap(), lok_naziv);

        let lok_json = serde_json::to_string(&lok).unwrap();
        Response::created(lok_json)
    });

    app.put("/turnir/:id", |_, params, _| {
        if let Some(turnir_id) = params.find("id") {
            println!("Put was sent successfully with id: {}", turnir_id);
        }

        Response::ok(String::from(""))
    });

    app.delete("/turnir/:id", |_, params, _| {
        if let Some(turnir_id) = params.find("id") {
            println!("Delete was sent successfully with id: {}", turnir_id);
        }

        Response::ok(String::from(""))
    });

    app.listen();
}
