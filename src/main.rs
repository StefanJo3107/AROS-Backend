use micro_backend_framework::controllers::lokacija_controller::LokacijaController;
use micro_backend_framework::controllers::partija_controller::PartijaController;
use micro_backend_framework::controllers::sahista_controller::SahistaController;
use micro_backend_framework::controllers::turnir_controller::TurnirController;
use micro_backend_framework::controllers::Controller;
use micro_backend_framework::establish_connection;
use micro_backend_framework::fianchetto::Fianchetto;
use std::sync::Arc;

fn main() {
    let mut app: Fianchetto = Fianchetto::new("127.0.0.1:1207", 4);
    let db_conn = Arc::new(establish_connection());

    LokacijaController::routes(&mut app, Arc::clone(&db_conn));
    TurnirController::routes(&mut app, Arc::clone(&db_conn));
    SahistaController::routes(&mut app, Arc::clone(&db_conn));
    PartijaController::routes(&mut app, Arc::clone(&db_conn));

    app.listen();
}
