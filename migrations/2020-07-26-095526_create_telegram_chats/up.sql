CREATE TABLE IF NOT EXISTS telegram_chats(
    id serial NOT NULL PRIMARY KEY,
    chat_id int NOT NULL DEFAULT 0,
    create_ts timestamptz NOT NULL DEFAULT now()
);

CREATE TABLE IF NOT EXISTS values(
    id serial NOT NULL PRIMARY KEY,
    name text NOT NULL,
    value jsonb
);

CREATE INDEX telegram_chats_chat_id_idx ON telegram_chats (chat_id);

INSERT INTO values (name, value) VALUES ('telegram_update_id', '0'::jsonb);
