-- SPDX-FileCopyrightText: 2023 Konrad Borowski <konrad@borowski.pw>
--
-- SPDX-License-Identifier: AGPL-3.0-or-later

CREATE TABLE languages (
    language_id serial PRIMARY KEY,
    priority int NOT NULL,
    name text NOT NULL,
    highlighter_mode text,
    mime text NOT NULL
);

INSERT INTO languages(priority, name, highlighter_mode, mime) VALUES
    (0, 'Plain text', NULL, 'text/plain'),
    (10, 'C', 'clike', 'text/x-csrc'),
    (10, 'C++', 'clike', 'text/x-c++src'),
    (10, 'C#', 'clike', 'text/x-csharp'),
    (10, 'Haskell', 'haskell', 'text/x-haskell'),
    (10, 'Java', 'clike', 'text/x-java'),
    (10, 'JavaScript', 'javascript', 'text/javascript'),
    (10, 'Jinja2', 'jinja2', 'text/jinja2'),
    (10, 'JSX', 'jsx', 'text/jsx'),
    (10, 'Markdown', 'markdown', 'text/x-markdown'),
    (10, 'Perl', 'perl', 'text/x-perl'),
    (10, 'Python 2', 'python', 'text/x-python'),
    (10, 'Python 3', 'python', 'text/x-python'),
    (10, 'Rust', 'rust', 'text/x-rustsrc'),
    (10, 'SQL', 'sql', 'text/x-sql'),
    (10, 'SQLite', 'sql', 'text/x-sqlite'),
    (10, 'TypeScript', 'javascript', 'application/typescript'),
    (10, 'TypeScript-JSX', 'jsx', 'text/typescript-jsx');

CREATE TABLE pastes (
    paste_id serial PRIMARY KEY,
    identifier text NOT NULL UNIQUE
);

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
