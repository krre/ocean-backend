ALTER TABLE mandels ADD COLUMN IF NOT EXISTS trash boolean NOT NULL DEFAULT false;

CREATE INDEX IF NOT EXISTS mandels_trash_idx ON mandels(trash);
