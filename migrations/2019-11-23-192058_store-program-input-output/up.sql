-- SPDX-FileCopyrightText: 2023 Konrad Borowski <konrad@borowski.pw>
--
-- SPDX-License-Identifier: AGPL-3.0-or-later

ALTER TABLE pastes
    ADD COLUMN stdin text NOT NULL DEFAULT '',
    ADD COLUMN exit_code integer,
    ADD COLUMN stdout text,
    ADD COLUMN stderr text;
