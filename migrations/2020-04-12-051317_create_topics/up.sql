CREATE TABLE IF NOT EXISTS topics (
    id serial NOT NULL PRIMARY KEY,
    title text NOT NULL,
    description text NOT NULL,
    create_ts timestamptz NOT NULL DEFAULT now(),
    update_ts timestamptz NOT NULL DEFAULT now()
);
