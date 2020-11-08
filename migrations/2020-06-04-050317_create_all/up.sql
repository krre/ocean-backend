CREATE TABLE IF NOT EXISTS user_groups (
    id serial NOT NULL PRIMARY KEY,
    name text,
    code text NOT NULL
);

INSERT INTO user_groups (name, code) VALUES ('Администраторы', 'admin');
INSERT INTO user_groups (name, code) VALUES ('Пользователи', 'user');
INSERT INTO user_groups (name, code) VALUES ('Анонимы', 'conspirator');
INSERT INTO user_groups (name, code) VALUES ('Незарегистрированные', 'fierce');

CREATE TABLE IF NOT EXISTS users (
    id serial NOT NULL PRIMARY KEY,
    name text,
    token text NOT NULL,
    group_id INT NOT NULL DEFAULT 2 REFERENCES user_groups(id) ON DELETE SET DEFAULT ON UPDATE CASCADE,
    create_ts timestamptz NOT NULL DEFAULT now(),
    update_ts timestamptz NOT NULL DEFAULT now()
);

INSERT INTO users (name, token, group_id) VALUES ('Администратор', '', 1);
INSERT INTO users (name, token, group_id) VALUES ('Аноним', '', 4);

CREATE TABLE IF NOT EXISTS mandels (
    id serial NOT NULL PRIMARY KEY,
    title text NOT NULL,
    description text NOT NULL,
    user_id INT NOT NULL REFERENCES users(id) ON DELETE SET DEFAULT ON UPDATE CASCADE,
    images jsonb NOT NULL DEFAULT '[]'::jsonb,
    videos jsonb NOT NULL DEFAULT '[]'::jsonb,
    links jsonb NOT NULL DEFAULT '[]'::jsonb,
    create_ts timestamptz NOT NULL DEFAULT now(),
    update_ts timestamptz NOT NULL DEFAULT now()
);
