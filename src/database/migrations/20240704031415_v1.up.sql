-- Add up migration script here
CREATE TABLE users (
    id BIGINT PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    discriminator SMALLINT,
    global_name VARCHAR(255),
    avatar VARCHAR(255),
    bot BOOLEAN NOT NULL,
    system BOOLEAN NOT NULL,
    mfa_enabled BOOLEAN NOT NULL,
    locale VARCHAR(255),
    flags BIGINT NOT NULL,
    premium_type VARCHAR(255) NOT NULL,
    public_flags BIGINT
);

CREATE TABLE guilds (
    id BIGINT PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    icon VARCHAR(255),
    icon_hash VARCHAR(255),
    splash VARCHAR(255),
    discovery_splash VARCHAR(255),
    owner_id BIGINT NOT NULL,
    afk_metadata JSON,
    widget_enabled BOOLEAN,
    widget_channel_id BIGINT,
    verification_level VARCHAR(255),
    default_message_notifications VARCHAR(255),
    explicit_content_filter VARCHAR(255),
    roles JSON,
    emojis JSON,
    FOREIGN KEY (owner_id) REFERENCES users(id)
);