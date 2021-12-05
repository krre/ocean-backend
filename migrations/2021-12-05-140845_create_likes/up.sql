CREATE TABLE IF NOT EXISTS likes(
    id serial NOT NULL PRIMARY KEY,
    user_id int NOT NULL REFERENCES users(id) ON DELETE CASCADE ON UPDATE CASCADE,
    comment_id int REFERENCES comments(id) ON DELETE CASCADE ON UPDATE CASCADE,
    post_id int REFERENCES forum_posts(id) ON DELETE CASCADE ON UPDATE CASCADE,
    value smallint NOT NULL,
    create_ts timestamptz NOT NULL DEFAULT now(),
    UNIQUE (user_id, comment_id),
    UNIQUE (user_id, post_id)
);

CREATE INDEX likes_user_id_idx ON likes(user_id);
CREATE INDEX likes_comment_id_idx ON likes(comment_id);
CREATE INDEX likes_post_id_idx ON likes(post_id);
