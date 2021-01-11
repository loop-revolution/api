ALTER TABLE users
ADD email VARCHAR(100) NOT NULL;

CREATE TABLE potential_users (
	id SERIAL PRIMARY KEY,
	email VARCHAR(100) NOT NULL,
	session_code VARCHAR NOT NULL,
	verification_code CHAR(6) NOT NULL,
	username VARCHAR(36) NOT NULL UNIQUE,
	password VARCHAR NOT NULL,
	created_at TIMESTAMP NOT NULL
);
