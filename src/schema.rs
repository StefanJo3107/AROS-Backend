table! {
    lokacija (id) {
        id -> Int4,
        naziv -> Varchar,
    }
}

table! {
    turnir (turnir_id) {
        turnir_id -> Int4,
        turnir_naziv -> Varchar,
        turnir_datum -> Varchar,
        broj_rundi -> Int4,
        lokacija_id -> Nullable<Int4>,
    }
}

joinable!(turnir -> lokacija (lokacija_id));

allow_tables_to_appear_in_same_query!(
    lokacija,
    turnir,
);
