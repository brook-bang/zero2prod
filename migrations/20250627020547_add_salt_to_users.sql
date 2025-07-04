-- Add migration script here
ALTER TABLE users Add COLUMN salt TEXT NOT NULL;