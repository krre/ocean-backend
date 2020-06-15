CREATE TABLE IF NOT EXISTS votes (
    id serial NOT NULL PRIMARY KEY,
    mandela_id int NOT NULL REFERENCES mandels(id) ON DELETE CASCADE ON UPDATE CASCADE,
    user_id int NOT NULL REFERENCES users(id) ON DELETE SET DEFAULT ON UPDATE CASCADE,
    vote smallint NOT NULL DEFAULT 0,
    create_ts timestamptz NOT NULL DEFAULT now()
);

CREATE INDEX votes_mandela_id_idx ON votes (mandela_id);
CREATE INDEX votes_user_id_idx ON votes (user_id);
CREATE INDEX votes_vote_idx ON votes (vote);
