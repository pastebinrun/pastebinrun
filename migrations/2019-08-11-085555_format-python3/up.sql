-- SPDX-FileCopyrightText: 2023 Konrad Borowski <konrad@borowski.pw>
--
-- SPDX-License-Identifier: AGPL-3.0-or-later

ALTER TABLE wrappers ADD COLUMN is_formatter boolean NOT NULL DEFAULT FALSE;

INSERT INTO wrappers (language_id, label, code, ordering, is_formatter)
    SELECT language_id, 'autopep8', 'autopep8-3 code', 2, TRUE
        FROM languages
        WHERE name = 'Python 3';
