-- SPDX-FileCopyrightText: 2023 Konrad Borowski <konrad@borowski.pw>
--
-- SPDX-License-Identifier: AGPL-3.0-or-later

ALTER TABLE pastes ADD COLUMN output text;
UPDATE pastes SET output =
    replace(stdout, E'\x7F', '\x7F\x7F')
        || CASE WHEN stderr = '' THEN '' ELSE concat(E'\x7FE', replace(stderr, E'\x7F', E'\x7F\x7F')) END;
ALTER TABLE pastes DROP COLUMN stdout, DROP COLUMN stderr;
