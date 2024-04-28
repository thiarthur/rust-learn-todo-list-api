-- Your SQL goes here
CREATE TABLE tasks (
    id SERIAL PRIMARY KEY,
    title VARCHAR NOT NULL,
    "description" TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT NOW() ,
    updated_at TIMESTAMP
);