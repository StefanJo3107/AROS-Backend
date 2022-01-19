table! {
    lokacija (id) {
        id -> Int4,
        naziv -> Varchar,
    }
}

table! {
    partija (partija_id) {
        partija_id -> Int4,
        runda -> Int4,
        beli_id -> Int4,
        crni_id -> Int4,
        pgn -> Varchar,
        rezultat -> Varchar,
        otvaranje -> Varchar,
        datum -> Nullable<Varchar>,
        turnir_id -> Int4,
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
        sahista_slika -> Nullable<Varchar>,
    }
}

table! {
    turnir (turnir_id) {
        turnir_id -> Int4,
        turnir_naziv -> Varchar,
        turnir_datum -> Varchar,
        broj_rundi -> Int4,
        lokacija_id -> Nullable<Int4>,
        turnir_slika -> Nullable<Varchar>,
    }
}

joinable!(partija -> turnir (turnir_id));
joinable!(sahista -> lokacija (lokacija_id));
joinable!(turnir -> lokacija (lokacija_id));

allow_tables_to_appear_in_same_query!(
    lokacija,
    partija,
    sahista,
    turnir,
);
