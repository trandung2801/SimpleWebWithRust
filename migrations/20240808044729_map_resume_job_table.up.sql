-- Add up migration script here
CREATE TABLE IF NOT EXISTS map_resume_job (
    id serial PRIMARY KEY,
    resume_id INTEGER NOT NULL,
    job_id INTEGER NOT NULL,

    created_on TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_on TIMESTAMP NOT NULL DEFAULT NOW()
);

INSERT INTO map_resume_job (resume_id, job_id)
values (1, 1);

INSERT INTO map_resume_job (resume_id, job_id)
values (2, 1);

INSERT INTO map_resume_job (resume_id, job_id)
values (3, 1);

INSERT INTO map_resume_job (resume_id, job_id)
values (1, 2);

INSERT INTO map_resume_job (resume_id, job_id)
values (2, 2);

INSERT INTO map_resume_job (resume_id, job_id)
values (3, 2);

INSERT INTO map_resume_job (resume_id, job_id)
values (1, 3);

INSERT INTO map_resume_job (resume_id, job_id)
values (2, 3);

INSERT INTO map_resume_job (resume_id, job_id)
values (3, 3);

