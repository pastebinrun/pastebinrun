ALTER TABLE pastes DROP COLUMN output;
ALTER TABLE pastes ADD COLUMN stdout text, ADD COLUMN stderr text;
