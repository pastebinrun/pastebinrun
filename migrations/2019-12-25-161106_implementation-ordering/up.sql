-- SPDX-FileCopyrightText: 2023 Konrad Borowski <konrad@borowski.pw>
--
-- SPDX-License-Identifier: AGPL-3.0-or-later

TRUNCATE implementation_wrappers;
DELETE FROM implementations;

ALTER TABLE implementations ADD COLUMN ordering int NOT NULL;
ALTER TABLE implementations ADD CONSTRAINT implementations_language_id_ordering_key UNIQUE (language_id, ordering);
