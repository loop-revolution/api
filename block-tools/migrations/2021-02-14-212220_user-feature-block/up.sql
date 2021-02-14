ALTER TABLE users
ADD featured_id BIGINT;
ALTER TABLE users
ADD CONSTRAINT fk_featured_id FOREIGN KEY (featured_id) REFERENCES blocks (id);
