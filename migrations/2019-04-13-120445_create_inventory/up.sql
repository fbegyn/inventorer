-- Your SQL goes here
CREATE TABLE items (
  id INTEGER PRIMARY KEY NOT NULL,
  name VARCHAR NOT NULL,
  location VARCHAR NOT NULL,
  team VARCHAR NOT NULL DEFAULT 'default',
  amount INTEGER,
  barcode INTEGER,
  for_rent BOOLEAN NOT NULL DEFAULT 'f'
)
