-- Your SQL goes here
CREATE TABLE turnir(
    turnir_id SERIAL PRIMARY KEY,
    turnir_naziv VARCHAR NOT NULL,
    turnir_datum VARCHAR NOT NULL,
    broj_rundi INT NOT NULL,
    lokacija_id INT,
    CONSTRAINT fk_lokacija1
    FOREIGN KEY(lokacija_id)
    REFERENCES lokacija(id)
    ON DELETE SET NULL
)