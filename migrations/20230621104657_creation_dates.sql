ALTER TABLE character ADD COLUMN creation_date TEXT NOT NULL DEFAULT '2000-01-01';
UPDATE character SET creation_date = '2023-06-20';
