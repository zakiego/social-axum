-- Add up migration script here
ALTER TABLE users ADD CONSTRAINT unique_username UNIQUE (username);
