CREATE TABLE IF NOT EXISTS comments (
    id serial NOT NULL PRIMARY KEY,
    mandela_id INT NOT NULL REFERENCES mandels(id) ON DELETE CASCADE ON UPDATE CASCADE,
    user_id INT NOT NULL REFERENCES users(id) ON DELETE SET DEFAULT ON UPDATE CASCADE,
    message text NOT NULL,
    create_ts timestamptz NOT NULL DEFAULT now(),
    update_ts timestamptz NOT NULL DEFAULT now()
);
