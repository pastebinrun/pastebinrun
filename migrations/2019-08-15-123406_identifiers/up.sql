-- SPDX-FileCopyrightText: 2023 Konrad Borowski <konrad@borowski.pw>
--
-- SPDX-License-Identifier: AGPL-3.0-or-later

ALTER TABLE languages ADD COLUMN identifier text;

UPDATE languages SET identifier = CASE name
    WHEN 'Plain text' THEN 'plain-text'
    WHEN 'C' THEN 'c'
    WHEN 'C++' THEN 'c-plus-plus'
    WHEN 'C#' THEN 'c-sharp'
    WHEN 'Haskell' THEN 'haskell'
    WHEN 'HTML' THEN 'html'
    WHEN 'Java' THEN 'java'
    WHEN 'JavaScript' THEN 'javascript'
    WHEN 'Jinja2' THEN 'jinja2'
    WHEN 'JSX' THEN 'jsx'
    WHEN 'Markdown' THEN 'markdown'
    WHEN 'Perl' THEN 'perl'
    WHEN 'Perl 6' THEN 'perl6'
    WHEN 'PHP' THEN 'php'
    WHEN 'PostgreSQL' THEN 'postgresql'
    WHEN 'Python 2' THEN 'python2'
    WHEN 'Python 3' THEN 'python3'
    WHEN 'Rust' THEN 'rust'
    WHEN 'Sh' THEN 'sh'
    WHEN 'SQL' THEN 'sql'
    WHEN 'SQLite' THEN 'sqlite'
    WHEN 'TypeScript' THEN 'typescript'
    WHEN 'TypeScript-JSX' THEN 'typescript-jsx'
    END;

CREATE INDEX ON languages (identifier);

ALTER TABLE languages ALTER COLUMN identifier SET NOT NULL;

ALTER TABLE wrappers ADD COLUMN identifier text;

UPDATE wrappers SET identifier = CASE label
    WHEN 'Run' THEN 'run'
    WHEN 'Format (black)' THEN 'format'
    WHEN 'Rustfmt' THEN 'format'
    WHEN 'ASM' THEN 'asm'
    END;

ALTER TABLE wrappers ALTER COLUMN identifier SET NOT NULL;

CREATE INDEX ON wrappers (identifier);
