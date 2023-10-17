-- SPDX-FileCopyrightText: 2023 Konrad Borowski <konrad@borowski.pw>
--
-- SPDX-License-Identifier: AGPL-3.0-or-later

TRUNCATE implementation_wrappers;
DELETE FROM implementations;

ALTER TABLE implementation_wrappers
    DROP CONSTRAINT implementation_wrappers_identifier_key,
    ADD CONSTRAINT implementation_wrappers_implementation_id_identifier_key
    UNIQUE (implementation_id, identifier);

WITH impl (identifier, implid, impllabel) AS (
    VALUES
        ('cpp', 'clang', 'Clang'),
        ('cpp', 'g-plus-plus', 'g++'),
        ('c', 'clang', 'Clang'),
        ('c', 'gcc', 'gcc'),
        ('python', 'cpython', 'CPython'),
        ('rust', 'rustc', 'rustc'),
        ('sqlite', 'sqlite', 'SQLite'),
        ('sh', 'sh', 'sh'),
        ('raku', 'rakudo', 'Rakudo'),
        ('perl', 'perl', 'perl')
)
INSERT INTO implementations (language_id, identifier, label)
    SELECT language_id, implid, impllabel
        FROM impl
        JOIN languages USING (identifier);

WITH implwrap (langidentifier, implidentifier, wrapidentifier, wraplabel, code, ordering, is_formatter, is_asm) AS (
    VALUES
        ('cpp', 'clang', 'run', 'Run', 'mv code code.cpp; clang++ %s code.cpp && ./a.out', 1, FALSE, FALSE),
        ('cpp', 'g-plus-plus', 'run', 'Run', 'mv code code.cpp; g++ %s code.cpp && ./a.out', 1, FALSE, FALSE),
        ('c', 'clang', 'run', 'Run', 'mv code code.c; clang %s code.c && ./a.out', 1, FALSE, FALSE),
        ('c', 'gcc', 'run', 'Run', 'mv code code.c; gcc %s code.c && ./a.out', 1, FALSE, FALSE),
        ('python', 'cpython', 'format', 'Format (black)', 'black code; cat code', 2, TRUE, FALSE),
        ('python', 'cpython', 'run', 'Run', 'python3 %s code', 1, FALSE, FALSE),
        ('sh', 'sh', 'run', 'Run', 'sh %s code', 1, FALSE, FALSE),
        ('sqlite', 'sqlite', 'run', 'Run', 'sqlite3 %s < code', 1, FALSE, FALSE),
        ('rust', 'rustc', 'run', 'Run', 'mv code code.rs && rustc %s code.rs && ./code', 1, FALSE, FALSE),
        ('raku', 'rakudo', 'run', 'Run', 'perl6 %s code', 1, FALSE, FALSE),
        ('rust', 'rustc', 'format', 'Rustfmt', 'rustfmt code; cat code', 3, TRUE, FALSE),
        ('rust', 'rustc', 'asm', 'ASM', 'rustc --emit asm --crate-type rlib code && cat code.s', 2, FALSE, TRUE),
        ('perl', 'perl', 'run', 'Run', 'perl %s code', 1, FALSE, FALSE)
)
INSERT INTO implementation_wrappers (implementation_id, identifier, label, code, ordering, is_formatter, is_asm)
    SELECT implementation_id, wrapidentifier, wraplabel, code, ordering, is_formatter, is_asm
        FROM implwrap
        JOIN implementations ON implidentifier = implementations.identifier
        JOIN languages ON implementations.language_id = languages.language_id AND langidentifier = languages.identifier;
