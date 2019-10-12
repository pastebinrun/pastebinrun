INSERT INTO implementations (language_id, identifier, label)
SELECT language_id, 'perl', 'perl'
    FROM languages
    WHERE identifier = 'perl';

INSERT INTO implementation_wrappers (implementation_id, identifier, label, code, ordering)
SELECT implementation_id, 'run', 'Run', 'perl %s code', 1
    FROM implementations
    JOIN languages USING (language_id)
    WHERE languages.identifier = 'perl';
