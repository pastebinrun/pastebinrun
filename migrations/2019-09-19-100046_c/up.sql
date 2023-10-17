-- SPDX-FileCopyrightText: 2023 Konrad Borowski <konrad@borowski.pw>
--
-- SPDX-License-Identifier: AGPL-3.0-or-later

INSERT INTO implementations (language_id, identifier, label)
SELECT language_id, 'clang', 'Clang'
    FROM languages
    WHERE identifier = 'c';

INSERT INTO implementations (language_id, identifier, label)
SELECT language_id, 'gcc', 'gcc'
    FROM languages
    WHERE identifier = 'c';

INSERT INTO implementation_wrappers (implementation_id, identifier, label, code, ordering)
SELECT implementation_id, 'run', 'Run', 'mv code code.c; clang %s code.c && ./a.out', 1
    FROM implementations
    JOIN languages USING (language_id)
    WHERE implementations.identifier = 'clang'
        AND languages.identifier = 'c';

INSERT INTO implementation_wrappers (implementation_id, identifier, label, code, ordering)
SELECT implementation_id, 'run', 'Run', 'mv code code.c; gcc %s code.c && ./a.out', 1
    FROM implementations
    WHERE identifier = 'gcc';
