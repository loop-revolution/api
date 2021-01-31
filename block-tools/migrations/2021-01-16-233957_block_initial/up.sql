CREATE TABLE blocks (
	id SERIAL PRIMARY KEY,
	block_type VARCHAR(36) NOT NULL,
	created_at TIMESTAMP NOT NULL,
	updated_at TIMESTAMP NOT NULL,
	block_data TEXT,
	owner_id INT NOT NULL,
	CONSTRAINT fk_owner_id FOREIGN KEY (owner_id) REFERENCES users (id)
);

CREATE TABLE properties (
	id SERIAL PRIMARY KEY,
	property_name VARCHAR(36) NOT NULL,
	parent_id INT NOT NULL,
	value_id INT NOT NULL,
	annotation VARCHAR(72),
	CONSTRAINT fk_parent_id FOREIGN KEY (parent_id) REFERENCES blocks (id),
	CONSTRAINT fk_value_id FOREIGN KEY (value_id) REFERENCES blocks (id)
);
