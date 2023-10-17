-- SPDX-FileCopyrightText: 2023 Konrad Borowski <konrad@borowski.pw>
--
-- SPDX-License-Identifier: AGPL-3.0-or-later

UPDATE pastes
    SET language_id = (
        SELECT language_id
        FROM languages
        WHERE identifier = 'python'
    )
    WHERE language_id = (
        SELECT language_id
        FROM languages
        WHERE identifier = 'python2'
    );

DELETE FROM languages WHERE identifier = 'python2';
