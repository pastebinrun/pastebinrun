ALTER TABLE wrappers DROP COLUMN is_formatter;

DELETE FROM wrappers WHERE label = 'autopep8';
