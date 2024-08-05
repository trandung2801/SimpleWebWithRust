-- Add up migration script here
CREATE TABLE IF NOT EXISTS resumes (
    id serial PRIMARY KEY,
    user_id INTEGER NOT NULL,
    url TEXT NOT NULL,
    status TEXT NOT NULL,

    created_on TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_on TIMESTAMP NOT NULL DEFAULT NOW()
);