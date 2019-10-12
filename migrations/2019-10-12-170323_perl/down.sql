DELETE FROM implementation_wrappers WHERE implementation_id IN (
    SELECT implementation_id
    FROM implementations
    JOIN languages USING (language_id)
    WHERE languages.identifier = 'perl'
);

DELETE FROM implementations WHERE implementation_id IN (
    SELECT implementation_id
    FROM implementations
    JOIN languages USING (language_id)
    WHERE languages.identifier = 'perl'
);
