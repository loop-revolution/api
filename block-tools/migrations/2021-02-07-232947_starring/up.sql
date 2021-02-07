ALTER TABLE blocks
ADD stars INT[] NOT NULL DEFAULT '{}';
ALTER TABLE blocks
ADD notif_enabled INT[] NOT NULL DEFAULT '{}';