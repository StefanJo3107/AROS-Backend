use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;
use micro_backend_framework::controllers::lokacija_controller::LokacijaController;
use micro_backend_framework::controllers::turnir_controller::TurnirController;
use micro_backend_framework::controllers::Controller;
use micro_backend_framework::establish_connection;
use micro_backend_framework::fianchetto::response::Response;
use micro_backend_framework::fianchetto::Fianchetto;

fn main() {
    let mut app: Fianchetto<Pool<ConnectionManager<PgConnection>>> =
        Fianchetto::new("127.0.0.1:1207", 4);
    app.set_db_connection(establish_connection());

    LokacijaController::routes(&mut app);
    TurnirController::routes(&mut app);

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
