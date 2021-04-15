CREATE TABLE comments (
	id BIGSERIAL PRIMARY KEY,
	author_id INT NOT NULL,
	content_id BIGINT NOT NULL,
	block_id BIGINT NOT NULL,
	stars INT[] NOT NULL,
	created_at TIMESTAMP NOT NULL,
	CONSTRAINT fk_author_id FOREIGN KEY (author_id) REFERENCES users (id),
	CONSTRAINT fk_content_id FOREIGN KEY (content_id) REFERENCES blocks (id)
);
