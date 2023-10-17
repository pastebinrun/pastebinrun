-- SPDX-FileCopyrightText: 2023 Konrad Borowski <konrad@borowski.pw>
--
-- SPDX-License-Identifier: AGPL-3.0-or-later

UPDATE languages SET identifier = CASE identifier
    WHEN 'plaintext' THEN 'plain-text'
    WHEN 'cpp' THEN 'c-plus-plus'
    WHEN 'csharp' THEN 'c-sharp'
    WHEN 'python' THEN 'python3'
    WHEN 'tsx' THEN 'typescript-jsx'
    ELSE identifier
END;
