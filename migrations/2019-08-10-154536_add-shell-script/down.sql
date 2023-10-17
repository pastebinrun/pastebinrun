-- SPDX-FileCopyrightText: 2023 Konrad Borowski <konrad@borowski.pw>
--
-- SPDX-License-Identifier: AGPL-3.0-or-later

DELETE FROM wrappers WHERE language_id = (SELECT language_id FROM languages WHERE name = 'Sh');
DELETE FROM languages WHERE name = 'Sh';
