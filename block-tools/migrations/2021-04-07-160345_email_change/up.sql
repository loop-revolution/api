CREATE TABLE email_confirm (
	id SERIAL PRIMARY KEY,
	new_email VARCHAR(100) NOT NULL,
	session_code VARCHAR NOT NULL,
	verification_code CHAR(6) NOT NULL,
	user_id SERIAL NOT NULL,
	created_at TIMESTAMP NOT NULL,
	CONSTRAINT fk_user_id FOREIGN KEY (user_id) REFERENCES users (id)
);
