-- Add up migration script here
CREATE TABLE guilds (
    id BIGINT PRIMARY KEY,
    name TEXT NOT NULL,
    icon TEXT,
    icon_hash TEXT,
    splash TEXT,
    owner_id BIGINT NOT NULL
);
