-- SPDX-FileCopyrightText: 2023 Konrad Borowski <konrad@borowski.pw>
--
-- SPDX-License-Identifier: AGPL-3.0-or-later

INSERT INTO wrappers (language_id, label, code, ordering)
    SELECT language_id, 'Run', 'sqlite3 < code', 1
        FROM languages
        WHERE name = 'SQLite';
