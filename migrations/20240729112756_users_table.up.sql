-- Add up migration script here
CREATE TABLE IF NOT EXISTS users (
    id serial PRIMARY KEY,
    email TEXT NOT NULL,
    password TEXT NOT NULL,
    company TEXT NOT NULL,
    is_admin BOOLEAN NOT NULL,

    created_on TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_on TIMESTAMP NOT NULL DEFAULT NOW()
);