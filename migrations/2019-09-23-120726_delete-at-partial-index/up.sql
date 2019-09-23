DROP INDEX pastes_delete_at_idx;
CREATE INDEX ON pastes (delete_at) WHERE delete_at IS NOT NULL;
