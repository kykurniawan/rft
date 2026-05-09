-- Add migration script here
CREATE TABLE IF NOT EXISTS users (
    id UUID PRIMARY KEY,
    name VARCHAR(255) NOT NULL UNIQUE,
    email VARCHAR(255) NOT NULL
);

-- Index
CREATE UNIQUE INDEX IF NOT EXISTS users_email_index ON users (email);