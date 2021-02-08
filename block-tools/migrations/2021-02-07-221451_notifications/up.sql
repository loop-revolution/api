CREATE TABLE notifications (
	id BIGSERIAL PRIMARY KEY,
	name VARCHAR NOT NULL,
	description VARCHAR NOT NULL,
	block_link BIGINT,
	recipients INT[] NOT NULL
)