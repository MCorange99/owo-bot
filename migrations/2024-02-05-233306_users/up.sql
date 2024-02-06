-- Your SQL goes here
CREATE TABLE IF NOT EXISTS users (
  uid SERIAL PRIMARY KEY NOT NULL,
  groups INTEGER[]
);