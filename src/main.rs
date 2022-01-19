use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;
use micro_backend_framework::controllers::lokacija_controller::LokacijaController;
use micro_backend_framework::controllers::sahista_controller::SahistaController;
use micro_backend_framework::controllers::turnir_controller::TurnirController;
use micro_backend_framework::controllers::Controller;
use micro_backend_framework::establish_connection;
use micro_backend_framework::fianchetto::Fianchetto;

fn main() {
    let mut app: Fianchetto<Pool<ConnectionManager<PgConnection>>> =
        Fianchetto::new("127.0.0.1:1207", 4);
    app.set_db_connection(establish_connection());

    LokacijaController::routes(&mut app);
    TurnirController::routes(&mut app);
    SahistaController::routes(&mut app);

    app.listen();
}
