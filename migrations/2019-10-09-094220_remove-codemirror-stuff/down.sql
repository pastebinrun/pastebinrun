ALTER TABLE languages ADD COLUMN highlighter_mode text;
ALTER TABLE languages ADD COLUMN mime text;

UPDATE languages SET
    highlighter_mode = CASE identifier
        WHEN 'c' THEN 'clike'
        WHEN 'c-plus-plus' THEN 'clike'
        WHEN 'c-sharp' THEN 'clike'
        WHEN 'haskell' THEN 'haskell'
        WHEN 'html' THEN 'htmlmixed'
        WHEN 'java' THEN 'clike'
        WHEN 'javascript' THEN 'javascript'
        WHEN 'jinja2' THEN 'jinja2'
        WHEN 'jsx' THEN 'jsx'
        WHEN 'markdown' THEN 'markdown'
        WHEN 'perl' THEN 'perl'
        WHEN 'php' THEN 'php'
        WHEN 'postgresql' THEN 'sql'
        WHEN 'python2' THEN 'python'
        WHEN 'python3' THEN 'python'
        WHEN 'rust' THEN 'rust'
        WHEN 'sh' THEN 'shell'
        WHEN 'sql' THEN 'sql'
        WHEN 'sqlite' THEN 'sql'
        WHEN 'typescript' THEN 'javascript'
        WHEN 'typescript-jsx' THEN 'jsx'
    END,
    mime = CASE identifier
        WHEN 'c' THEN 'text/x-csrc'
        WHEN 'c-plus-plus' THEN 'text/x-c++src'
        WHEN 'c-sharp' THEN 'text/x-csharp'
        WHEN 'haskell' THEN 'text/x-haskell'
        WHEN 'html' THEN 'text/html'
        WHEN 'java' THEN 'text/x-java'
        WHEN 'javascript' THEN 'text/javascript'
        WHEN 'jinja2' THEN 'text/jinja2'
        WHEN 'jsx' THEN 'text/jsx'
        WHEN 'markdown' THEN 'text/x-markdown'
        WHEN 'perl' THEN 'text/x-perl'
        WHEN 'perl6' THEN 'text/x-perl6'
        WHEN 'php' THEN 'application/x-httpd-php'
        WHEN 'plain-text' THEN 'text/plain'
        WHEN 'postgresql' THEN 'text/x-pgsql'
        WHEN 'python2' THEN 'text/x-python'
        WHEN 'python3' THEN 'text/x-python'
        WHEN 'rust' THEN 'text/x-rustsrc'
        WHEN 'sh' THEN 'text/x-sh'
        WHEN 'sql' THEN 'text/x-sql'
        WHEN 'sqlite' THEN 'text/x-sqlite'
        WHEN 'typescript' THEN 'application/typescript'
        WHEN 'typescript-jsx' THEN 'text/typescript-jsx'
    END;

ALTER TABLE languages ALTER COLUMN mime SET NOT NULL;
