ALTER TABLE pastes
    ADD COLUMN created_at timestamp with time zone NOT NULL DEFAULT now(),
    ADD COLUMN language_id integer REFERENCES languages,
    ADD COLUMN paste text;

UPDATE pastes AS p SET
    created_at = (
        SELECT created_at
            FROM paste_revisions AS pr
            WHERE p.paste_id = pr.paste_id
    ),
    language_id = (
        SELECT language_id
            FROM paste_revisions AS pr
            NATURAL JOIN paste_contents
            WHERE p.paste_id = pr.paste_id
    ),
    paste = (
        SELECT paste
            FROM paste_revisions AS pr
            NATURAL JOIN paste_contents
            WHERE p.paste_id = pr.paste_id
    );

ALTER TABLE pastes
    ALTER COLUMN language_id SET NOT NULL,
    ALTER COLUMN paste SET NOT NULL;

DROP TABLE paste_contents;
DROP TABLE paste_revisions;
