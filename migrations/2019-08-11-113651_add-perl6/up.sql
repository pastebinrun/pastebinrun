WITH inserted_language AS (
    INSERT INTO languages(priority, name, highlighter_mode, mime) VALUES
        (10, 'Perl 6', NULL, 'text/x-perl6')
        RETURNING language_id
)
INSERT INTO wrappers (language_id, label, code, ordering)
    SELECT language_id, 'Run', 'perl6 code', 1
        FROM inserted_language;
