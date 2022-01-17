CREATE TABLE IF NOT EXISTS account
(
    id          SERIAL PRIMARY KEY,
    username    VARCHAR(64) UNIQUE,
    password    VARCHAR(64) 
);