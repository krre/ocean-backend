CREATE TABLE IF NOT EXISTS forum_posts(
    id serial NOT NULL PRIMARY KEY,
    topic_id int NOT NULL REFERENCES forum_topics(id) ON DELETE CASCADE ON UPDATE CASCADE,
    user_id int NOT NULL REFERENCES users(id) ON DELETE CASCADE ON UPDATE CASCADE,
    post text NOT NULL,
    create_ts timestamptz NOT NULL DEFAULT now(),
    update_ts timestamptz NOT NULL DEFAULT now()
);

CREATE INDEX forum_posts_topic_id_idx ON forum_posts(topic_id);
CREATE INDEX forum_posts_user_id_idx ON forum_posts(user_id);
CREATE INDEX forum_posts_create_ts_idx ON forum_posts(create_ts);
