-- SPDX-FileCopyrightText: 2023 Konrad Borowski <konrad@borowski.pw>
--
-- SPDX-License-Identifier: AGPL-3.0-or-later

WITH insertable_implementations (
  identifier,
  label,
  implementation_identifier
) AS (
  VALUES
    ('perl6', 'Rakudo', 'rakudo'),
    ('python3', 'CPython', 'cpython'),
    ('rust', 'rustc', 'rustc'),
    ('sh', 'sh', 'sh'),
    ('sqlite', 'SQLite', 'sqlite')
),
insert_implementations AS (
  INSERT INTO
    implementations (language_id, label, identifier)
  SELECT
    language_id,
    label,
    implementation_identifier
  FROM
    insertable_implementations
    JOIN languages USING (identifier)
    RETURNING implementation_id, language_id
)
INSERT INTO
  implementation_wrappers (
    implementation_id,
    identifier,
    label,
    code,
    ordering,
    is_formatter,
    is_asm
  )
SELECT
  implementation_id,
  identifier,
  label,
  code,
  ordering,
  is_formatter,
  is_asm
FROM
  insert_implementations
  JOIN shared_wrappers USING (language_id);
DROP TABLE shared_wrappers;
