ALTER TABLE mandels ADD COLUMN IF NOT EXISTS automatic_trash boolean NOT NULL DEFAULT true;

CREATE INDEX IF NOT EXISTS mandels_automatic_trash_idx ON mandels(automatic_trash);
