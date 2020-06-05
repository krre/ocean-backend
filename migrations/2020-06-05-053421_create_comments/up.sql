CREATE TABLE IF NOT EXISTS comments (
    id serial NOT NULL PRIMARY KEY,
    message text NOT NULL,
    user_id INT NOT NULL REFERENCES users(id) ON DELETE SET DEFAULT ON UPDATE CASCADE,
    create_ts timestamptz NOT NULL DEFAULT now(),
    update_ts timestamptz NOT NULL DEFAULT now()
);
