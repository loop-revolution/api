ALTER TABLE blocks
ADD perm_full INT[] NOT NULL DEFAULT '{}';
ALTER TABLE blocks
ADD perm_edit INT[] NOT NULL DEFAULT '{}';
ALTER TABLE blocks
ADD perm_view INT[] NOT NULL DEFAULT '{}';
