ALTER TABLE updates
DROP COLUMN seen;
ALTER TABLE users
ADD latest_update_seen_id INT;
ALTER TABLE users
ADD CONSTRAINT fk_latest_update_seen_id FOREIGN KEY (latest_update_seen_id) REFERENCES updates (id);
