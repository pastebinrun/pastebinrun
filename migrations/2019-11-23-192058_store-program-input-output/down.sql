-- SPDX-FileCopyrightText: 2023 Konrad Borowski <konrad@borowski.pw>
--
-- SPDX-License-Identifier: AGPL-3.0-or-later

ALTER TABLE pastes
    DROP COLUMN stdin,
    DROP COLUMN exit_code,
    DROP COLUMN stdout,
    DROP COLUMN stderr;
