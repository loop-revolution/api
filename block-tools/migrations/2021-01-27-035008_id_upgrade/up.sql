ALTER TABLE blocks
ALTER COLUMN id TYPE BIGINT;

ALTER TABLE properties
ALTER COLUMN id TYPE BIGINT,
ALTER COLUMN parent_id TYPE BIGINT,
ALTER COLUMN value_id TYPE BIGINT;