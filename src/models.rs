use super::schema::{lokacija, sahista, turnir};
use diesel::{Identifiable, Queryable};
use serde::{Deserialize, Serialize};

#[derive(Identifiable, Queryable, Serialize)]
#[table_name = "lokacija"]
pub struct Lokacija {
    pub id: i32,
    pub naziv: String,
}

#[derive(Insertable)]
#[table_name = "lokacija"]
pub struct NewLokacija<'a> {
    pub naziv: &'a str,
}

#[derive(Identifiable, Queryable, Serialize, Associations)]
#[primary_key(turnir_id)]
#[belongs_to(Lokacija)]
#[table_name = "turnir"]
pub struct Turnir {
    pub turnir_id: i32,
    pub turnir_naziv: String,
    pub turnir_datum: String,
    pub broj_rundi: i32,
    pub lokacija_id: Option<i32>,
}

#[derive(Insertable, Deserialize)]
#[table_name = "turnir"]
pub struct NewTurnir {
    pub turnir_naziv: String,
    pub turnir_datum: String,
    pub broj_rundi: i32,
    pub lokacija_id: Option<i32>,
}

#[derive(Identifiable, Queryable, Serialize, Associations)]
#[primary_key(sahista_id)]
#[belongs_to(Lokacija)]
#[table_name = "sahista"]
pub struct Sahista {
    pub sahista_id: i32,
    pub titula_fide: String,
    pub elo: i32,
    pub ime: String,
    pub prezime: String,
    pub lokacija_id: Option<i32>,
}

#[derive(Insertable, Deserialize)]
#[table_name = "sahista"]
pub struct NewSahista {
    pub titula_fide: String,
    pub elo: i32,
    pub ime: String,
    pub prezime: String,
    pub lokacija_id: Option<i32>,
}
