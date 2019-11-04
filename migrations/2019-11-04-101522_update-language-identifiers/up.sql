UPDATE languages SET identifier = CASE identifier
    WHEN 'plain-text' THEN 'plaintext'
    WHEN 'c-plus-plus' THEN 'cpp'
    WHEN 'c-sharp' THEN 'csharp'
    WHEN 'python3' THEN 'python'
    WHEN 'typescript-jsx' THEN 'tsx'
    ELSE identifier
END;
