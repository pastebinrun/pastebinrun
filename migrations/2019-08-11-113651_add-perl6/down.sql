DELETE FROM wrappers WHERE language_id = (SELECT language_id FROM languages WHERE name = 'Perl 6');
DELETE FROM languages WHERE name = 'Perl 6';
