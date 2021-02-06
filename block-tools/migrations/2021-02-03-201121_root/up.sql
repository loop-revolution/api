ALTER TABLE users
ADD root_id BIGINT;
ALTER TABLE users
ADD CONSTRAINT fk_root_id FOREIGN KEY (root_id) REFERENCES blocks (id);