ALTER TABLE updates
ADD seen INT[] NOT NULL DEFAULT '{}';
ALTER TABLE users
DROP COLUMN latest_update_seen_id;
