-- Add up migration script here
ALTER TABLE users ADD COLUMN access_token VARCHAR(255);