-- Add up migration script here
CREATE TABLE IF NOT EXISTS resumes (
    id serial PRIMARY KEY,
    user_id INTEGER NOT NULL,
    email TEXT NOT NULL,
    url TEXT,
    is_delete BOOLEAN NOT NULL ,

    created_on TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_on TIMESTAMP NOT NULL DEFAULT NOW()
);