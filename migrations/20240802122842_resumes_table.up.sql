-- Add up migration script here
CREATE TABLE IF NOT EXISTS resumes (
    id serial PRIMARY KEY,
    user_id INTEGER NOT NULL,
    email TEXT NOT NULL,
    url TEXT,
    is_delete BOOLEAN NOT NULL ,

    created_on TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_on TIMESTAMP NOT NULL DEFAULT NOW()
);

INSERT INTO resumes (user_id, email, url, is_delete)
values (7 , 'user2@gmail.com', 'abcxyz', false);

INSERT INTO resumes (user_id, email, url, is_delete)
values (7 , 'user2@gmail.com', 'abcxyz', false);

INSERT INTO resumes (user_id, email, url, is_delete)
values (6, 'user1@gmail.com', 'abcxyz', false);

INSERT INTO resumes (user_id, email, url, is_delete)
values (6, 'user1@gmail.com', 'abcxyz', true);

