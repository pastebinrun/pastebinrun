ALTER TABLE pastes
    ADD COLUMN stdin text NOT NULL DEFAULT '',
    ADD COLUMN exit_code integer,
    ADD COLUMN stdout text,
    ADD COLUMN stderr text;
