ALTER TABLE forum_topics ADD COLUMN IF NOT EXISTS type smallint NOT NULL DEFAULT 0;
ALTER TABLE forum_topics ADD COLUMN IF NOT EXISTS poll_selection_type smallint;

CREATE TABLE IF NOT EXISTS forum_poll_answers (
    id serial NOT NULL PRIMARY KEY,
    topic_id int NOT NULL REFERENCES forum_topics(id) ON DELETE CASCADE ON UPDATE CASCADE,
    answer text NOT NULL
);

CREATE INDEX forum_poll_answers_topic_id_idx ON forum_posts(topic_id);

CREATE TABLE IF NOT EXISTS forum_poll_votes (
    id serial NOT NULL PRIMARY KEY,
    answer_id int NOT NULL REFERENCES forum_poll_answers(id) ON DELETE CASCADE ON UPDATE CASCADE,
    user_id int NOT NULL REFERENCES users(id) ON DELETE SET DEFAULT ON UPDATE CASCADE,
    create_ts timestamptz NOT NULL DEFAULT now()
);

CREATE INDEX forum_poll_votes_answer_id_idx ON forum_poll_votes(answer_id);
CREATE INDEX forum_poll_votes_user_id_idx ON forum_poll_votes(user_id);
