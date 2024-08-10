-- Add up migration script here
CREATE TABLE IF NOT EXISTS users (
    id serial PRIMARY KEY,
    email TEXT NOT NULL,
    password TEXT NOT NULL,
    company_id INTEGER NOT NULL,
    role_id INTEGER NOT NULL,
    is_delete BOOLEAN NOT NULL ,

    created_on TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_on TIMESTAMP NOT NULL DEFAULT NOW()
);

INSERT INTO users (email, password, company_id, role_id, is_delete)
values ('admin@gmail.com', '$argon2i$v=19$m=4096,t=3,p=1$x+BlYXdrM9ndT+P42TYNt7UNOnn6xslz75HzSjY9+A8$iIXBtO+glnyIT27QjDilaoTNYTNsYq38M6gzK3XqpxE', 0, 1, false);

INSERT INTO users (email, password, company_id, role_id, is_delete)
values ('admin1@gmail.com', '$argon2i$v=19$m=4096,t=3,p=1$x+BlYXdrM9ndT+P42TYNt7UNOnn6xslz75HzSjY9+A8$iIXBtO+glnyIT27QjDilaoTNYTNsYq38M6gzK3XqpxE', 0, 1, false);

INSERT INTO users (email, password, company_id, role_id, is_delete)
values ('hr1@gmail.com', '$argon2i$v=19$m=4096,t=3,p=1$x+BlYXdrM9ndT+P42TYNt7UNOnn6xslz75HzSjY9+A8$iIXBtO+glnyIT27QjDilaoTNYTNsYq38M6gzK3XqpxE', 1, 3, false);

INSERT INTO users (email, password, company_id, role_id, is_delete)
values ('hr2@gmail.com', '$argon2i$v=19$m=4096,t=3,p=1$x+BlYXdrM9ndT+P42TYNt7UNOnn6xslz75HzSjY9+A8$iIXBtO+glnyIT27QjDilaoTNYTNsYq38M6gzK3XqpxE', 1, 3, false);

INSERT INTO users (email, password, company_id, role_id, is_delete)
values ('hr3@gmail.com', '$argon2i$v=19$m=4096,t=3,p=1$x+BlYXdrM9ndT+P42TYNt7UNOnn6xslz75HzSjY9+A8$iIXBtO+glnyIT27QjDilaoTNYTNsYq38M6gzK3XqpxE', 1, 3, false);

INSERT INTO users (email, password, company_id, role_id, is_delete)
values ('user1@gmail.com', '$argon2i$v=19$m=4096,t=3,p=1$x+BlYXdrM9ndT+P42TYNt7UNOnn6xslz75HzSjY9+A8$iIXBtO+glnyIT27QjDilaoTNYTNsYq38M6gzK3XqpxE', 0, 2, false);

INSERT INTO users (email, password, company_id, role_id, is_delete)
values ('user2@gmail.com', '$argon2i$v=19$m=4096,t=3,p=1$x+BlYXdrM9ndT+P42TYNt7UNOnn6xslz75HzSjY9+A8$iIXBtO+glnyIT27QjDilaoTNYTNsYq38M6gzK3XqpxE', 0, 2, false);

INSERT INTO users (email, password, company_id, role_id, is_delete)
values ('user3@gmail.com', '$argon2i$v=19$m=4096,t=3,p=1$x+BlYXdrM9ndT+P42TYNt7UNOnn6xslz75HzSjY9+A8$iIXBtO+glnyIT27QjDilaoTNYTNsYq38M6gzK3XqpxE', 0, 2, false);
