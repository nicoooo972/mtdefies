CREATE TABLE users (
                       id SERIAL NOT NULL PRIMARY KEY,
                       pseudo TEXT NOT NULL,
                       pw TEXT NOT NULL,
                       email TEXT NOT NULL,
                       created_at TIMESTAMP NOT NULL
);
