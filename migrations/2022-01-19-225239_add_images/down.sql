-- This file should undo anything in `up.sql`
ALTER TABLE sahista
DROP COLUMN IF EXISTS sahista_slika;

ALTER TABLE turnir
DROP COLUMN IF EXISTS turnir_slika;
