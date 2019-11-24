DROP INDEX languages_identifier_idx;

ALTER TABLE languages ADD CONSTRAINT languages_identifier_key UNIQUE (identifier);
