CREATE TABLE IF NOT EXISTS account
(
    id                SERIAL PRIMARY KEY,
    username    CHARACTER(64),
    password    CHARACTER(64)
);