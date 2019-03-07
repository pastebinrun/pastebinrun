ALTER TABLE languages ADD COLUMN is_markdown boolean NOT NULL DEFAULT FALSE;
UPDATE languages SET is_markdown = TRUE WHERE name = 'Markdown';
