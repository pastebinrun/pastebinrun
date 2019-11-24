ALTER TABLE languages DROP CONSTRAINT languages_identifier_key;
CREATE INDEX ON languages (identifier);
