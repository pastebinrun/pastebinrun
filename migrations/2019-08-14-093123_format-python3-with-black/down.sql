-- SPDX-FileCopyrightText: 2023 Konrad Borowski <konrad@borowski.pw>
--
-- SPDX-License-Identifier: AGPL-3.0-or-later

UPDATE wrappers
    SET label = 'autopep8', code = 'autopep8-3 code'
    WHERE label = 'Format (black)';
