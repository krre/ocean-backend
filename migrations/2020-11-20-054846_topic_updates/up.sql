ALTER TABLE forum_topics ADD COLUMN IF NOT EXISTS last_post_id int REFERENCES forum_posts(id) ON DELETE SET DEFAULT ON UPDATE CASCADE;
ALTER TABLE forum_topics ADD COLUMN IF NOT EXISTS last_post_create_ts timestamptz;

CREATE INDEX forum_topics_last_post_create_ts_idx ON forum_topics(last_post_create_ts);
