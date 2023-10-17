-- SPDX-FileCopyrightText: 2023 Konrad Borowski <konrad@borowski.pw>
--
-- SPDX-License-Identifier: AGPL-3.0-or-later

ALTER TABLE wrappers RENAME TO shared_wrappers;

CREATE TABLE implementations (
    implementation_id serial PRIMARY KEY,
    language_id integer NOT NULL REFERENCES languages,
    identifier text NOT NULL,
    label text NOT NULL,
    UNIQUE (language_id, identifier)
);

CREATE TABLE implementation_wrappers (
    implementation_wrapper_id serial PRIMARY KEY,
    implementation_id integer NOT NULL REFERENCES implementations,
    identifier text NOT NULL,
    label text NOT NULL,
    code text NOT NULL,
    ordering int NOT NULL,
    is_formatter boolean NOT NULL DEFAULT false,
    is_asm boolean NOT NULL DEFAULT false,
    UNIQUE (implementation_id, identifier)
);

INSERT INTO implementations (language_id, identifier, label)
SELECT language_id, 'clang', 'Clang'
    FROM languages
    WHERE identifier = 'c-plus-plus';

INSERT INTO implementations (language_id, identifier, label)
SELECT language_id, 'g-plus-plus', 'g++'
    FROM languages
    WHERE identifier = 'c-plus-plus';

INSERT INTO implementation_wrappers (implementation_id, identifier, label, code, ordering)
SELECT implementation_id, 'run', 'Run', 'mv code code.cpp; clang++ %s code.cpp && ./a.out', 1
    FROM implementations
    WHERE identifier = 'clang';

INSERT INTO implementation_wrappers (implementation_id, identifier, label, code, ordering)
SELECT implementation_id, 'run', 'Run', 'mv code code.cpp; g++ %s code.cpp && ./a.out', 1
    FROM implementations
    WHERE identifier = 'g-plus-plus';
