ALTER TABLE wrappers ADD COLUMN is_asm boolean NOT NULL DEFAULT FALSE;

UPDATE wrappers SET code = 'python3 %s code' WHERE code = 'python3 code';
UPDATE wrappers SET code = 'sh %s code' WHERE code = 'sh code';
UPDATE wrappers SET code = 'sqlite3 %s < code' WHERE code = 'sqlite3 < code';
UPDATE wrappers SET code = 'mv code code.rs && rustc %s code.rs && ./code' WHERE code = 'mv code code.rs && rustc code.rs && ./code';
UPDATE wrappers SET code = 'perl6 %s code' WHERE code = 'perl6 code';

UPDATE wrappers SET ordering = 3 WHERE label = 'Rustfmt';

INSERT INTO wrappers (language_id, label, code, ordering, is_asm)
    SELECT language_id, 'ASM', 'rustc --emit asm --crate-type rlib code && cat code.s', 2, TRUE
        FROM languages WHERE name = 'Rust';
