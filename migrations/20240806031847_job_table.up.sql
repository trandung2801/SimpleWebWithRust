-- Add up migration script here
CREATE TABLE IF NOT EXISTS jobs (
    id serial PRIMARY KEY,
    name Text NOT NULL,
    company_id INTEGER NOT NULL,
    location TEXT NOT NULL,
    quantity Integer NOT NULL,
    salary integer not null,
    level TEXT NOT NULL,
    description TEXT NOT NULL,

    created_on TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_on TIMESTAMP NOT NULL DEFAULT NOW(),
);