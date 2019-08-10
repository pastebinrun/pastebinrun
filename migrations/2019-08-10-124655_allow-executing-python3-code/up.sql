CREATE TABLE wrappers (
    wrapper_id serial PRIMARY KEY,
    language_id int NOT NULL REFERENCES languages,
    label text NOT NULL,
    code text NOT NULL,
    ordering int NOT NULL,
    UNIQUE (language_id, ordering)
);

INSERT INTO wrappers (language_id, label, code, ordering)
    SELECT language_id, 'Run', 'python3 code', 1
        FROM languages
        WHERE name = 'Python 3';
