-- SPDX-FileCopyrightText: 2023 Konrad Borowski <konrad@borowski.pw>
--
-- SPDX-License-Identifier: AGPL-3.0-or-later

ALTER TABLE paste_revisions
    DROP CONSTRAINT paste_revisions_paste_id_fkey,
    ADD CONSTRAINT paste_revisions_paste_id_fkey
        FOREIGN KEY (paste_id)
        REFERENCES pastes (paste_id) ON DELETE CASCADE;

ALTER TABLE paste_contents
    DROP CONSTRAINT paste_contents_paste_revision_id_fkey,
    ADD CONSTRAINT paste_contents_paste_revision_id_fkey
        FOREIGN KEY (paste_revision_id)
        REFERENCES paste_revisions (paste_revision_id) ON DELETE CASCADE;

ALTER TABLE pastes ADD COLUMN delete_at timestamp with time zone;

CREATE INDEX ON pastes (delete_at);
