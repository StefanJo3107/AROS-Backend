-- Your SQL goes here
CREATE TABLE partija(
    partija_id SERIAL PRIMARY KEY,
    runda INT NOT NULL,
    beli_id INT NOT NULL,
    crni_id INT NOT NULL,
    pgn VARCHAR NOT NULL, 
    rezultat VARCHAR NOT NULL,
    otvaranje VARCHAR NOT NULL,
    datum VARCHAR,
    turnir_id INT NOT NULL,
    CONSTRAINT fk_beli
    FOREIGN KEY(beli_id)
    REFERENCES sahista(sahista_id)
    ON DELETE CASCADE,
    CONSTRAINT fk_crni
    FOREIGN KEY(crni_id)
    REFERENCES sahista(sahista_id)
    ON DELETE CASCADE,
    CONSTRAINT fk_turnir
    FOREIGN KEY(turnir_id)
    REFERENCES turnir(turnir_id)
    ON DELETE CASCADE
)