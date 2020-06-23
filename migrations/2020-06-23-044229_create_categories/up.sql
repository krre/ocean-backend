CREATE TABLE IF NOT EXISTS categories (
    id serial NOT NULL PRIMARY KEY,
    mandela_id int NOT NULL REFERENCES mandels(id) ON DELETE CASCADE ON UPDATE CASCADE,
    number smallint NOT NULL DEFAULT 0
);

CREATE INDEX categories_mandela_id_idx ON categories (mandela_id);
CREATE INDEX categories_number_idx ON categories (number);
