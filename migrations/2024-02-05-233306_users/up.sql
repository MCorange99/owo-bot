-- Your SQL goes here
CREATE TABLE IF NOT EXISTS users (
  uid BIGINT PRIMARY KEY NOT NULL,
  groups INTEGER[]
);