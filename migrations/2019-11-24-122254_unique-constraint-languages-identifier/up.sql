-- SPDX-FileCopyrightText: 2023 Konrad Borowski <konrad@borowski.pw>
--
-- SPDX-License-Identifier: AGPL-3.0-or-later

DROP INDEX languages_identifier_idx;

ALTER TABLE languages ADD CONSTRAINT languages_identifier_key UNIQUE (identifier);
