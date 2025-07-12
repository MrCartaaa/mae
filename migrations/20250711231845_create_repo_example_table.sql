BEGIN;
SELECT create_table(
  'repoexample',
  '
  value INTEGER NOT NULL,
  string_value TEXT NOT NULL,
  ');
COMMIT;
