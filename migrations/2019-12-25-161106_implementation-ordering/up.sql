TRUNCATE implementation_wrappers;
DELETE FROM implementations;

ALTER TABLE implementations ADD COLUMN ordering int NOT NULL;
ALTER TABLE implementations ADD CONSTRAINT implementations_language_id_ordering_key UNIQUE (language_id, ordering);
