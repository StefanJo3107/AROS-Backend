use std::sync::Arc;

use diesel::{
    r2d2::{ConnectionManager, Pool},
    PgConnection,
};

use crate::fianchetto::Fianchetto;

pub mod lokacija_controller;
pub mod partija_controller;
pub mod sahista_controller;
pub mod turnir_controller;

pub trait Controller {
    fn routes(app: &mut Fianchetto, conn_pool: Arc<Pool<ConnectionManager<PgConnection>>>);
}
