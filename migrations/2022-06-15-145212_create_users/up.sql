CREATE TABLE users (
  id SERIAL PRIMARY KEY,
  username TEXT NOT NULL UNIQUE,
  hashpass TEXT NOT NULL,
  salt TEXT NOT NULL,
  created TIMESTAMP NOT NULL DEFAULT NOW(),
  stopped TIMESTAMP NOT NULL DEFAULT NOW(),
  attempts INTEGER NOT NULL DEFAULT 1
)