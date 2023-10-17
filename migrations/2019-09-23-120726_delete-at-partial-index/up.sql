-- SPDX-FileCopyrightText: 2023 Konrad Borowski <konrad@borowski.pw>
--
-- SPDX-License-Identifier: AGPL-3.0-or-later

DROP INDEX pastes_delete_at_idx;
CREATE INDEX ON pastes (delete_at) WHERE delete_at IS NOT NULL;
