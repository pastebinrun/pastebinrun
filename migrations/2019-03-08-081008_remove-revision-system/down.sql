CREATE TABLE paste_revisions (
    paste_revision_id serial PRIMARY KEY,
    created_at timestamp with time zone NOT NULL DEFAULT now(),
    paste_id int NOT NULL REFERENCES pastes
);

CREATE INDEX ON paste_revisions(paste_id, created_at DESC);

CREATE TABLE paste_contents (
    paste_content_id serial PRIMARY KEY,
    paste_revision_id int NOT NULL REFERENCES paste_revisions,
    language_id int NOT NULL REFERENCES languages,
    paste text NOT NULL
);

CREATE INDEX ON paste_contents(paste_revision_id);

INSERT INTO paste_revisions (created_at, paste_id)
    SELECT created_at, paste_id FROM pastes;

INSERT INTO paste_contents (paste_revision_id, language_id, paste)
    SELECT paste_revision_id, language_id, paste FROM pastes NATURAL JOIN paste_revisions;

ALTER TABLE pastes
    DROP COLUMN created_at,
    DROP COLUMN language_id,
    DROP COLUMN paste;
