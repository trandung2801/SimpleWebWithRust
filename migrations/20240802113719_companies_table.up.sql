-- Add up migration script here
CREATE TABLE IF NOT EXISTS companies (
    id serial PRIMARY KEY,
    email TEXT NOT NULL,
    name TEXT NOT NULL,
    address TEXT NOT NULL,
    description TEXT NOT NULL,

    created_on TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_on TIMESTAMP NOT NULL DEFAULT NOW()
);