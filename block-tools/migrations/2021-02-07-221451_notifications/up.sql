CREATE TABLE notifications (
	id BIGSERIAL PRIMARY KEY,
	name VARCHAR(36) NOT NULL,
	description VARCHAR(36) NOT NULL,
	block_link BIGINT,
	recipients INT[] NOT NULL
)