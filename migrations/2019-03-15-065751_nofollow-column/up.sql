ALTER TABLE pastes ADD COLUMN no_follow boolean NOT NULL DEFAULT TRUE;

UPDATE pastes SET no_follow = FALSE WHERE identifier = 'about';
