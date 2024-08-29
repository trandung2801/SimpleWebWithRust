-- Add up migration script here
CREATE TABLE IF NOT EXISTS users (
    id serial PRIMARY KEY,
    email TEXT NOT NULL,
    hash_password TEXT NOT NULL,
    company_id INTEGER NOT NULL,
    role_id INTEGER NOT NULL,
    is_delete BOOLEAN NOT NULL ,

    created_on TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_on TIMESTAMP NOT NULL DEFAULT NOW()
);

