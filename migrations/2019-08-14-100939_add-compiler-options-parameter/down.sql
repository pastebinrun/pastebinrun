-- SPDX-FileCopyrightText: 2023 Konrad Borowski <konrad@borowski.pw>
--
-- SPDX-License-Identifier: AGPL-3.0-or-later

ALTER TABLE wrappers DROP COLUMN is_asm;

UPDATE wrappers SET code = 'python3 code' WHERE code = 'python3 %s code';
UPDATE wrappers SET code = 'sh code' WHERE code = 'sh %s code';
UPDATE wrappers SET code = 'sqlite3 < code' WHERE code = 'sqlite3 %s < code';
UPDATE wrappers SET code = 'mv code code.rs && rustc code.rs && ./code' WHERE code = 'mv code code.rs && rustc %s code.rs && ./code';
UPDATE wrappers SET code = 'perl6 code' WHERE code = 'perl6 %s code';

DELETE FROM wrappers WHERE language_id = (SELECT language_id FROM languages WHERE name = 'Rust') AND label = 'ASM';

UPDATE wrappers SET ordering = 2 WHERE label = 'Rustfmt';
