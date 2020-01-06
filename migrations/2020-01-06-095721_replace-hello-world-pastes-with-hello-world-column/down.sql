ALTER TABLE languages DROP COLUMN hello_world, ADD COLUMN hello_world_paste_id int REFERENCES pastes;
