ALTER TABLE paste_revisions
    DROP CONSTRAINT paste_revisions_paste_id_fkey,
    ADD CONSTRAINT paste_revisions_paste_id_fkey
        FOREIGN KEY (paste_id)
        REFERENCES pastes (paste_id);

ALTER TABLE paste_contents
    DROP CONSTRAINT paste_contents_paste_revision_id_fkey,
    ADD CONSTRAINT paste_contents_paste_revision_id_fkey
        FOREIGN KEY (paste_revision_id)
        REFERENCES paste_revisions (paste_revision_id);

ALTER TABLE pastes DROP COLUMN delete_at;
