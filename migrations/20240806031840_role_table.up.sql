-- Add up migration script here
CREATE TABLE IF NOT EXISTS roles (
    id serial PRIMARY KEY,
    role Text NOT NULL,
    is_delete BOOLEAN NOT NULL ,

    created_on TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_on TIMESTAMP NOT NULL DEFAULT NOW()
);

INSERT INTO roles (role, is_delete)
values ('admin', false);

INSERT INTO roles (role, is_delete)
values ('user', false);

INSERT INTO roles (role, is_delete)
values ('hr', false);