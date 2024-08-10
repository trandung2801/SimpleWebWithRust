-- Add up migration script here
CREATE TABLE IF NOT EXISTS companies (
    id serial PRIMARY KEY,
    name TEXT NOT NULL,
    email TEXT NOT NULL,
    address TEXT,
    description TEXT,
    is_delete BOOLEAN NOT NULL ,

    created_on TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_on TIMESTAMP NOT NULL DEFAULT NOW()
);

INSERT INTO companies (name, email, address, description, is_delete)
values ('123', '12next@gmail.com', 'So 4 Nguyen Trai', 'Web', false);

INSERT INTO companies (name, email, address, description, is_delete)
values ('1234', '1234next@gmail.com', 'So 4 Nguyen Trai', 'Web', false);

INSERT INTO companies (name, email, address, description, is_delete)
values ('12345', '12345next@gmail.com', 'So 4 Nguyen Trai', 'Web', false);