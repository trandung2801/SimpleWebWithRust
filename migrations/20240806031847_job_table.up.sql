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

INSERT INTO jobs (job_name, company_id, location, quantity, salary, job_level, description, is_delete)
values ('Intern Smart Contract', '1', 'So 2 Pham Van Bach', 4, 2000000, 'Intern' , 'Intern Smart Contract', false);

INSERT INTO jobs (job_name, company_id, location, quantity, salary, job_level, description, is_delete)
values ('Junior Smart Contract', '1', 'So 2 Pham Van Bach', 3, 8000000, 'Junior', 'Junior Smart Contract', false);

INSERT INTO jobs (job_name, company_id, location, quantity, salary, job_level, description, is_delete)
values ('BA Smart Contract', '2', 'So 4 Nguyen Trai', 2, 15000000, 'Junior', 'BA Smart Contract', false);

INSERT INTO jobs (job_name, company_id, location, quantity, salary, job_level, description, is_delete)
values ('Senior Smart Contract', '2', 'So 4 Nguyen Trai', 2, 25000000, 'Senior', 'Senior Smart Contract', false);