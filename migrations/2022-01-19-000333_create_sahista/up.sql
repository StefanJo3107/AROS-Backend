-- Your SQL goes here
CREATE TABLE sahista(
    sahista_id SERIAL PRIMARY KEY,
    titula_fide VARCHAR NOT NULL,
    elo INT NOT NULL,
    ime VARCHAR NOT NULL,
    prezime VARCHAR NOT NULL,
    lokacija_id INT,
    CONSTRAINT fk_lokacija2
    FOREIGN KEY(lokacija_id)
    REFERENCES lokacija(id)
    ON DELETE SET NULL
)