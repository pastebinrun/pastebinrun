CREATE TABLE wrappers (
  wrapper_id serial PRIMARY KEY,
  language_id int NOT NULL REFERENCES languages,
  label text NOT NULL,
  code text NOT NULL,
  ordering int NOT NULL,
  is_formatter boolean NOT NULL DEFAULT FALSE,
  is_asm boolean NOT NULL DEFAULT false,
  identifier text NOT NULL,
  UNIQUE (language_id, ordering)
);
ALTER TABLE
  wrappers RENAME TO shared_wrappers;
WITH moved_wrappers AS (
    INSERT INTO
      shared_wrappers (
        language_id,
        label,
        code,
        ordering,
        is_formatter,
        is_asm,
        identifier
      )
    SELECT
      language_id,
      implementation_wrappers.label,
      code,
      ordering,
      is_formatter,
      is_asm,
      implementation_wrappers.identifier
    FROM
      implementation_wrappers
      JOIN implementations USING (implementation_id)
      JOIN languages USING (language_id)
    WHERE
      languages.identifier IN ('perl6', 'python3', 'rust', 'sh', 'sqlite')
    RETURNING language_id
  ),
  deleted_implementation_wrappers AS (
    DELETE FROM
      implementation_wrappers
    WHERE
      implementation_wrapper_id IN (
        SELECT
          implementation_wrapper_id
        FROM
          moved_wrappers
          JOIN implementations USING (language_id)
          JOIN implementation_wrappers USING (implementation_id)
      )
    RETURNING implementation_id
  )
DELETE FROM
  implementations
WHERE
  implementation_id IN (
    SELECT
      implementation_id
    FROM
      deleted_implementation_wrappers
  );
