-- Add up migration script here
CREATE TABLE users (
    id BIGINT PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    discriminator INT,
    global_name VARCHAR(255),
    avatar VARCHAR(255),
    bot BOOLEAN
)

