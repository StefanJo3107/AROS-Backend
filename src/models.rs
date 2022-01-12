use super::schema::lokacija;
use diesel::Queryable;
use serde::Serialize;

#[derive(Queryable, Serialize)]
pub struct Lokacija {
    pub id: i32,
    pub naziv: String,
}

#[derive(Insertable)]
#[table_name = "lokacija"]
pub struct NewLokacija<'a> {
    pub naziv: &'a str,
}
