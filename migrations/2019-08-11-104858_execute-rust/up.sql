-- SPDX-FileCopyrightText: 2023 Konrad Borowski <konrad@borowski.pw>
--
-- SPDX-License-Identifier: AGPL-3.0-or-later

INSERT INTO wrappers (language_id, label, code, ordering)
    SELECT language_id, 'Run', 'mv code code.rs && rustc code.rs && ./code', 1
        FROM languages
        WHERE name = 'Rust';

INSERT INTO wrappers (language_id, label, code, ordering, is_formatter)
    SELECT language_id, 'Rustfmt', 'rustfmt code; cat code', 2, TRUE
        FROM languages
        WHERE name = 'Rust';
