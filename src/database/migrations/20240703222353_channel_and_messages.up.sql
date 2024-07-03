-- Add up migration script here
CREATE TABLE channels (
    id BIGINT PRIMARY KEY,
    guild_id BIGINT NOT NULL,
    name TEXT NOT NULL,
    position INT NOT NULL,
    nsfw BOOLEAN NOT NULL
);

CREATE TABLE messages (
    id BIGINT PRIMARY KEY,
    channel_id BIGINT NOT NULL,
    author BIGINT NOT NULL,
    content TEXT NOT NULL,
    timestamp TIMESTAMP NOT NULL,
    pinned BOOLEAN NOT NULL,
    guild_id BIGINT NOT NULL
);