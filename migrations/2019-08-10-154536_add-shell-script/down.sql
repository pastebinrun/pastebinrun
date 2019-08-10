DELETE FROM wrappers WHERE language_id = (SELECT language_id FROM languages WHERE name = 'Sh');
DELETE FROM languages WHERE name = 'Sh';
