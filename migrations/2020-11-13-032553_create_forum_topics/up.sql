CREATE TABLE IF NOT EXISTS forum_topics(
    id serial NOT NULL PRIMARY KEY,
    section_id int NOT NULL REFERENCES forum_sections(id) ON DELETE CASCADE ON UPDATE CASCADE,
    user_id int NOT NULL REFERENCES users(id) ON DELETE CASCADE ON UPDATE CASCADE,
    name text NOT NULL,
    create_ts timestamptz NOT NULL DEFAULT now(),
    update_ts timestamptz NOT NULL DEFAULT now()
);

CREATE INDEX forum_topics_section_id_idx ON forum_topics(section_id);
CREATE INDEX forum_topics_user_id_idx ON forum_topics(user_id);
CREATE INDEX forum_topics_create_ts_idx ON forum_topics(create_ts);
