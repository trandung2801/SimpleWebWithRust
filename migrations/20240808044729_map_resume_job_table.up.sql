-- Add up migration script here
CREATE TABLE IF NOT EXISTS map_resume_job (
    id serial PRIMARY KEY,
    resume_id INTEGER NOT NULL,
    job_id INTEGER NOT NULL,

    created_on TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_on TIMESTAMP NOT NULL DEFAULT NOW()
);
