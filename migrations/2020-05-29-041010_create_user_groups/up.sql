CREATE TABLE IF NOT EXISTS user_groups (
    id serial NOT NULL PRIMARY KEY,
    name text,
    code text NOT NULL
);

INSERT INTO user_groups (name, code) VALUES ('Администраторы', 'admin');
INSERT INTO user_groups (name, code) VALUES ('Пользователи', 'user');
INSERT INTO user_groups (name, code) VALUES ('Конспирологи', 'conspirator');

ALTER TABLE users ADD COLUMN IF NOT EXISTS group_id INT NOT NULL DEFAULT 2
    REFERENCES user_groups(id)
    ON DELETE SET DEFAULT
    ON UPDATE CASCADE;
