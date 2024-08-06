-- Add up migration script here
CREATE TABLE IF NOT EXISTS roles (
    id serial PRIMARY KEY,
    role Text NOT NULL,

    created_on TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_on TIMESTAMP NOT NULL DEFAULT NOW()
);