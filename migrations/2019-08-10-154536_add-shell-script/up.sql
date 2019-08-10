INSERT INTO languages(priority, name, highlighter_mode, mime) VALUES
    (10, 'Sh', 'shell', 'text/x-sh');

INSERT INTO wrappers (language_id, label, code, ordering)
    SELECT language_id, 'Run', 'sh code', 1
        FROM languages
        WHERE name = 'Sh';
