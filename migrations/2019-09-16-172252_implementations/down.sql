-- SPDX-FileCopyrightText: 2023 Konrad Borowski <konrad@borowski.pw>
--
-- SPDX-License-Identifier: AGPL-3.0-or-later

ALTER TABLE shared_wrappers RENAME TO wrappers;
DROP TABLE implementation_wrappers;
DROP TABLE implementations;
