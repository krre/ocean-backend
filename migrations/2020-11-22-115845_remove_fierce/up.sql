START TRANSACTION;

UPDATE users SET name = concat(name, id) WHERE group_id = 3;
ALTER TABLE users ALTER COLUMN name SET NOT NULL;

UPDATE users SET group_id = 2 WHERE group_id = 3;
UPDATE users SET group_id = 3 WHERE group_id = 4;

UPDATE user_groups SET code = 'anonym'
WHERE code = 'conspirator';

DELETE FROM user_groups WHERE code = 'fierce';

COMMIT;
