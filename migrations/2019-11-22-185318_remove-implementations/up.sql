TRUNCATE implementation_wrappers;
DELETE FROM implementations;

ALTER TABLE implementation_wrappers
    DROP CONSTRAINT implementation_wrappers_implementation_id_identifier_key,
    ADD CONSTRAINT implementation_wrappers_identifier_key
    UNIQUE (identifier);
