-- Add up migration script here
CREATE TABLE IF NOT EXISTS jobs (
    id serial PRIMARY KEY,
    job_name TEXT NOT NULL,
    company_id INTEGER NOT NULL,
    location TEXT NOT NULL,
    quantity INTEGER NOT NULL,
    salary INTEGER NOT NULL,
    job_level TEXT NOT NULL,
    description TEXT NOT NULL,
    is_delete BOOLEAN NOT NULL,

    created_on TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_on TIMESTAMP NOT NULL DEFAULT NOW()
);