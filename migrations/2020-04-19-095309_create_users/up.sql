CREATE TABLE IF NOT EXISTS users (
    id serial NOT NULL PRIMARY KEY,
    name text NOT NULL,
    create_ts timestamptz NOT NULL DEFAULT now(),
    update_ts timestamptz NOT NULL DEFAULT now()
);

ALTER TABLE topics ADD COLUMN IF NOT EXISTS user_id INT NOT NULL
    REFERENCES users(id)
    ON DELETE SET DEFAULT
    ON UPDATE CASCADE;
