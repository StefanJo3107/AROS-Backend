table! {
    lokacija (id) {
        id -> Int4,
        naziv -> Varchar,
    }
}

table! {
    sahista (sahista_id) {
        sahista_id -> Int4,
        titula_fide -> Varchar,
        elo -> Int4,
        ime -> Varchar,
        prezime -> Varchar,
        lokacija_id -> Nullable<Int4>,
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

joinable!(sahista -> lokacija (lokacija_id));
joinable!(turnir -> lokacija (lokacija_id));

allow_tables_to_appear_in_same_query!(
    lokacija,
    sahista,
    turnir,
);
