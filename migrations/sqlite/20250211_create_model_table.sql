
CREATE TABLE IF NOT EXISTS models
(
  id          INTEGER NOT NULL PRIMARY KEY,
  key       VARCHAR,
  value     VARCHAR,
  created_date   TIMESTAMP,
  updated_date   TIMESTAMP
);
