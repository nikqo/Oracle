-- Add up migration script here
CREATE TABLE roles (
    id BIGINT PRIMARY KEY,
    guild_id BIGINT NOT NULL,
    mentionable BOOLEAN NOT NULL,
    name TEXT NOT NULL,
    position INT NOT NULL
);
