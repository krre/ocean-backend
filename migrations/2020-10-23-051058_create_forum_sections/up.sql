CREATE TABLE IF NOT EXISTS forum_sections(
    id serial NOT NULL PRIMARY KEY,
    category_id int NOT NULL REFERENCES forum_categories(id) ON DELETE CASCADE ON UPDATE CASCADE,
    name text NOT NULL,
    order_index smallint NOT NULL,
    create_ts timestamptz NOT NULL DEFAULT now(),
    update_ts timestamptz NOT NULL DEFAULT now()
);

CREATE INDEX forum_sections_category_id_idx ON forum_sections (category_id);
