-- DROP TABLE IF EXISTS eclipse_lvls;

-- CREATE TABLE eclipse_lvls (
--   user_name TEXT PRIMARY KEY,
--   lvls INTEGER[] NOT NULL
-- );



CREATE TABLE IF NOT EXISTS eclipse_lvls (
  user_name TEXT PRIMARY KEY,
  lvls INTEGER[] NOT NULL
);
