-- SPDX-FileCopyrightText: 2023 Konrad Borowski <konrad@borowski.pw>
--
-- SPDX-License-Identifier: AGPL-3.0-or-later

UPDATE wrappers
    SET label = 'Format (black)', code = 'black code; cat code'
    WHERE label = 'autopep8';
