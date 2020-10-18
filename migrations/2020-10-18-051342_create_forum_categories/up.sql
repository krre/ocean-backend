CREATE TABLE IF NOT EXISTS forum_categories(
    id serial NOT NULL PRIMARY KEY,
    name text NOT NULL,
    order_index smallint NOT NULL,
    create_ts timestamptz NOT NULL DEFAULT now(),
    update_ts timestamptz NOT NULL DEFAULT now()
);
