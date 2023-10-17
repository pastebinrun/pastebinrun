-- SPDX-FileCopyrightText: 2023 Konrad Borowski <konrad@borowski.pw>
--
-- SPDX-License-Identifier: AGPL-3.0-or-later

ALTER TABLE languages ADD COLUMN is_markdown boolean NOT NULL DEFAULT FALSE;
UPDATE languages SET is_markdown = TRUE WHERE identifier = 'markdown';
