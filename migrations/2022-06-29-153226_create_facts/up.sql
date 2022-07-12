CREATE TABLE facts (
  id SERIAL PRIMARY KEY,
  title TEXT,
  body TEXT NOT NULL,
  link TEXT
)