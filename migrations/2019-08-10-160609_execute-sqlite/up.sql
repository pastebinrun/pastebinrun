INSERT INTO wrappers (language_id, label, code, ordering)
    SELECT language_id, 'Run', 'sqlite3 < code', 1
        FROM languages
        WHERE name = 'SQLite';
