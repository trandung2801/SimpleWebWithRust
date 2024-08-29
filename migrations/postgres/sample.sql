INSERT INTO users (email, hash_password, company_id, role_id, is_delete)
values
    ('admin@gmail.com', '$argon2i$v=19$m=4096,t=3,p=1$x+BlYXdrM9ndT+P42TYNt7UNOnn6xslz75HzSjY9+A8$iIXBtO+glnyIT27QjDilaoTNYTNsYq38M6gzK3XqpxE', 0, 1, false),
    ('admin1@gmail.com', '$argon2i$v=19$m=4096,t=3,p=1$x+BlYXdrM9ndT+P42TYNt7UNOnn6xslz75HzSjY9+A8$iIXBtO+glnyIT27QjDilaoTNYTNsYq38M6gzK3XqpxE', 0, 1, false),
    ('hr1@gmail.com', '$argon2i$v=19$m=4096,t=3,p=1$x+BlYXdrM9ndT+P42TYNt7UNOnn6xslz75HzSjY9+A8$iIXBtO+glnyIT27QjDilaoTNYTNsYq38M6gzK3XqpxE', 1, 3, false),
    ('hr2@gmail.com', '$argon2i$v=19$m=4096,t=3,p=1$x+BlYXdrM9ndT+P42TYNt7UNOnn6xslz75HzSjY9+A8$iIXBtO+glnyIT27QjDilaoTNYTNsYq38M6gzK3XqpxE', 1, 3, false),
    ('hr3@gmail.com', '$argon2i$v=19$m=4096,t=3,p=1$x+BlYXdrM9ndT+P42TYNt7UNOnn6xslz75HzSjY9+A8$iIXBtO+glnyIT27QjDilaoTNYTNsYq38M6gzK3XqpxE', 1, 3, false),
    ('user1@gmail.com', '$argon2i$v=19$m=4096,t=3,p=1$x+BlYXdrM9ndT+P42TYNt7UNOnn6xslz75HzSjY9+A8$iIXBtO+glnyIT27QjDilaoTNYTNsYq38M6gzK3XqpxE', 0, 2, false),
    ('user2@gmail.com', '$argon2i$v=19$m=4096,t=3,p=1$x+BlYXdrM9ndT+P42TYNt7UNOnn6xslz75HzSjY9+A8$iIXBtO+glnyIT27QjDilaoTNYTNsYq38M6gzK3XqpxE', 0, 2, false),
    ('user3@gmail.com', '$argon2i$v=19$m=4096,t=3,p=1$x+BlYXdrM9ndT+P42TYNt7UNOnn6xslz75HzSjY9+A8$iIXBtO+glnyIT27QjDilaoTNYTNsYq38M6gzK3XqpxE', 0, 2, false);

INSERT INTO companies (name, email, address, description, is_delete)
values
    ('123', '12next@gmail.com', 'So 4 Nguyen Trai', 'Web', false),
    ('1234', '1234next@gmail.com', 'So 4 Nguyen Trai', 'Web', false),
    ('12345', '12345next@gmail.com', 'So 4 Nguyen Trai', 'Web', false);

INSERT INTO resumes (user_id, email, url, is_delete)
values
    (7 , 'user2@gmail.com', 'abcxyz', false),
    (7 , 'user2@gmail.com', 'abcxyz', false),
    (6, 'user1@gmail.com', 'abcxyz', false),
    (6, 'user1@gmail.com', 'abcxyz', true);

INSERT INTO roles (role, is_delete)
values
    ('admin', false),
    ('user', false),
    ('hr', false);

INSERT INTO jobs (job_name, company_id, location, quantity, salary, job_level, description, is_delete)
values
    ('Intern Smart Contract', '1', 'So 2 Pham Van Bach', 4, 2000000, 'Intern' , 'Intern Smart Contract', false),
    ('Junior Smart Contract', '1', 'So 2 Pham Van Bach', 3, 8000000, 'Junior', 'Junior Smart Contract', false),
    ('BA Smart Contract', '2', 'So 4 Nguyen Trai', 2, 15000000, 'Junior', 'BA Smart Contract', false),
    ('Senior Smart Contract', '2', 'So 4 Nguyen Trai', 2, 25000000, 'Senior', 'Senior Smart Contract', false);

INSERT INTO map_resume_job (resume_id, job_id)
values
    (1, 1),
    (2, 1),
    (3, 1),
    (1, 2),
    (2, 2),
    (3, 2),
    (1, 3),
    (2, 3),
    (3, 3)


