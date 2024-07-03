use sqlx::postgres::PgPool;
use super::model::{DbUser, DbGuild, DbGuildChannel, DbMessage, DbRole};
use async_trait::async_trait;

// Crud Operations for users in model.rs

#[async_trait]
pub trait CrudOperation<T> {
    async fn create(pool: &PgPool, item: &T) -> Result<T, sqlx::Error>;
    async fn read(pool: &PgPool, id: i64) -> Result<T, sqlx::Error>;
    async fn update(pool: &PgPool, item: &T) -> Result<T, sqlx::Error>;
    async fn delete(pool: &PgPool, id: i64) -> Result<T, sqlx::Error>;
}

// DbUser implementation for CrudOperation
#[async_trait]
impl CrudOperation<DbUser> for DbUser {
    async fn create(pool: &PgPool, user: &DbUser) -> Result<DbUser, sqlx::Error> {
        let row = sqlx::query_as!(
            DbUser,
            r#"
            INSERT INTO users (id, name, discriminator, global_name, avatar, bot)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING id, name, discriminator, global_name, avatar, bot
            "#,
            user.id.into(),
            user.name,
            user.discriminator,
            user.global_name,
            user.avatar,
            user.bot,
        )
        .fetch_one(pool)
        .await?;

        Ok(row)
    }

    async fn read(pool: &PgPool, id: i64) -> Result<DbUser, sqlx::Error> {
        let row = sqlx::query_as!(
            DbUser,
            r#"
            SELECT id, name, discriminator, global_name, avatar, bot
            FROM users
            WHERE id = $1
            "#,
            id
        )
        .fetch_one(pool)
        .await?;

        Ok(row)
    }

    async fn update(pool: &PgPool, user: &DbUser) -> Result<DbUser, sqlx::Error> {
        let row = sqlx::query_as!(
            DbUser,
            r#"
            UPDATE users
            SET name = $2, discriminator = $3, global_name = $4, avatar = $5, bot = $6
            WHERE id = $1
            RETURNING id, name, discriminator, global_name, avatar, bot
            "#,
            user.id.into(),
            user.name,
            user.discriminator,
            user.global_name,
            user.avatar,
            user.bot,
        )
        .fetch_one(pool)
        .await?;

        Ok(row)
    }

    async fn delete(pool: &PgPool, id: i64) -> Result<DbUser, sqlx::Error> {
        let row = sqlx::query_as!(
            DbUser,
            r#"
            DELETE FROM users
            WHERE id = $1
            RETURNING id, name, discriminator, global_name, avatar, bot
            "#,
            id
        )
        .fetch_one(pool)
        .await?;

        Ok(row)
    }
}

// DbGuild implementation for CrudOperation
#[async_trait]
impl CrudOperation<DbGuild> for DbGuild {
    async fn create(pool: &PgPool, guild: &DbGuild) -> Result<DbGuild, sqlx::Error> {
        let row = sqlx::query_as!(
            DbGuild,
            r#"
            INSERT INTO guilds (id, name, icon, icon_hash, splash, owner_id)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING id, name, icon, icon_hash, splash, owner_id
            "#,
            guild.id.into(),
            guild.name,
            guild.icon,
            guild.icon_hash,
            guild.splash,
            guild.owner_id.into()
        )
        .fetch_one(pool)
        .await?;

        Ok(row)
    }

    async fn read(pool: &PgPool, id: i64) -> Result<DbGuild, sqlx::Error> {
        let row = sqlx::query_as!(
            DbGuild,
            r#"
            SELECT id, name, icon, icon_hash, splash, owner_id
            FROM guilds
            WHERE id = $1
            "#,
            id
        )
        .fetch_one(pool)
        .await?;

        Ok(row)
    }

    async fn update(pool: &PgPool, guild: &DbGuild) -> Result<DbGuild, sqlx::Error> {
        let row = sqlx::query_as!(
            DbGuild,
            r#"
            UPDATE guilds
            SET name = $2, icon = $3, icon_hash = $4, splash = $5, owner_id = $6
            WHERE id = $1
            RETURNING id, name, icon, icon_hash, splash, owner_id
            "#,
            guild.id.into(),
            guild.name,
            guild.icon,
            guild.icon_hash,
            guild.splash,
            guild.owner_id.into()
        )
        .fetch_one(pool)
        .await?;

        Ok(row)
    }

    async fn delete(pool: &PgPool, id: i64) -> Result<DbGuild, sqlx::Error> {
        let row = sqlx::query_as!(
            DbGuild,
            r#"
            DELETE FROM guilds
            WHERE id = $1
            RETURNING id, name, icon, icon_hash, splash, owner_id
            "#,
            id
        )
        .fetch_one(pool)
        .await?;

        Ok(row)
    }
}

// DbGuildChannel implementation for CrudOperation
#[async_trait]
impl CrudOperation<DbGuildChannel> for DbGuildChannel {
    async fn create(pool: &PgPool, channel: &DbGuildChannel) -> Result<DbGuildChannel, sqlx::Error> {
        let row = sqlx::query_as!(
            DbGuildChannel,
            r#"
            INSERT INTO channels (id, guild_id, name, position, nsfw)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, guild_id, name, position, nsfw
            "#,
            channel.id.into(),
            channel.guild_id.into(),
            channel.name,
            channel.position,
            channel.nsfw
        )
        .fetch_one(pool)
        .await?;

        Ok(row)
    }

    async fn read(pool: &PgPool, id: i64) -> Result<DbGuildChannel, sqlx::Error> {
        let row = sqlx::query_as!(
            DbGuildChannel,
            r#"
            SELECT id, guild_id, name, position, nsfw
            FROM channels
            WHERE id = $1
            "#,
            id
        )
        .fetch_one(pool)
        .await?;

        Ok(row)
    }

    async fn update(pool: &PgPool, channel: &DbGuildChannel) -> Result<DbGuildChannel, sqlx::Error> {
        let row = sqlx::query_as!(
            DbGuildChannel,
            r#"
            UPDATE channels
            SET guild_id = $2, name = $3, position = $4, nsfw = $5
            WHERE id = $1
            RETURNING id, guild_id, name, position, nsfw
            "#,
            channel.id.into(),
            channel.guild_id.into(),
            channel.name,
            channel.position,
            channel.nsfw
        )
        .fetch_one(pool)
        .await?;

        Ok(row)
    }

    async fn delete(pool: &PgPool, id: i64) -> Result<DbGuildChannel, sqlx::Error> {
        let row = sqlx::query_as!(
            DbGuildChannel,
            r#"
            DELETE FROM channels
            WHERE id = $1
            RETURNING id, guild_id, name, position, nsfw
            "#,
            id
        )
        .fetch_one(pool)
        .await?;

        Ok(row)
    }
}

// DbMessage implementation for CrudOperation
#[async_trait]
impl CrudOperation<DbMessage> for DbMessage {
    async fn create(pool: &PgPool, message: &DbMessage) -> Result<DbMessage, sqlx::Error> {
        let row = sqlx::query_as!(
            DbMessage,
            r#"
            INSERT INTO messages (id, channel_id, author, content, timestamp, pinned, guild_id)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING id, channel_id, author, content, timestamp, pinned, guild_id
            "#,
            message.id.into(),
            message.channel_id.into(),
            message.author.into(),
            message.content,
            message.timestamp,
            message.pinned,
            message.guild_id.into()
        )
        .fetch_one(pool)
        .await?;

        Ok(row)
    }

    async fn read(pool: &PgPool, id: i64) -> Result<DbMessage, sqlx::Error> {
        let row = sqlx::query_as!(
            DbMessage,
            r#"
            SELECT id, channel_id, author, content, timestamp, pinned, guild_id
            FROM messages
            WHERE id = $1
            "#,
            id
        )
        .fetch_one(pool)
        .await?;

        Ok(row)
    }

    async fn update(pool: &PgPool, message: &DbMessage) -> Result<DbMessage, sqlx::Error> {
        let row = sqlx::query_as!(
            DbMessage,
            r#"
            UPDATE messages
            SET channel_id = $2, author = $3, content = $4, timestamp = $5, pinned = $6, guild_id = $7
            WHERE id = $1
            RETURNING id, channel_id, author, content, timestamp, pinned, guild_id
            "#,
            message.id.into(),
            message.channel_id.into(),
            message.author.into(),
            message.content,
            message.timestamp,
            message.pinned,
            message.guild_id.into()
        )
        .fetch_one(pool)
        .await?;

        Ok(row)
    }

    async fn delete(pool: &PgPool, id: i64) -> Result<DbMessage, sqlx::Error> {
        let row = sqlx::query_as!(
            DbMessage,
            r#"
            DELETE FROM messages
            WHERE id = $1
            RETURNING id, channel_id, author, content, timestamp, pinned, guild_id
            "#,
            id
        )
        .fetch_one(pool)
        .await?;

        Ok(row)
    }
}

// DbRole implementation for CrudOperation
#[async_trait]
impl CrudOperation<DbRole> for DbRole {
    async fn create(pool: &PgPool, role: &DbRole) -> Result<DbRole, sqlx::Error> {
        let row = sqlx::query_as!(
            DbRole,
            r#"
            INSERT INTO roles (id, guild_id, mentionable, name, position)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, guild_id, mentionable, name, position
            "#,
            role.id.into(),
            role.guild_id.into(),
            role.mentionable,
            role.name,
            role.position
        )
        .fetch_one(pool)
        .await?;

        Ok(row)
    }

    async fn read(pool: &PgPool, id: i64) -> Result<DbRole, sqlx::Error> {
        let row = sqlx::query_as!(
            DbRole,
            r#"
            SELECT id, guild_id, mentionable, name, position
            FROM roles
            WHERE id = $1
            "#,
            id
        )
        .fetch_one(pool)
        .await?;

        Ok(row)
    }

    async fn update(pool: &PgPool, role: &DbRole) -> Result<DbRole, sqlx::Error> {
        let row = sqlx::query_as!(
            DbRole,
            r#"
            UPDATE roles
            SET guild_id = $2, mentionable = $3, name = $4, position = $5
            WHERE id = $1
            RETURNING id, guild_id, mentionable, name, position
            "#,
            role.id.into(),
            role.guild_id.into(),
            role.mentionable,
            role.name,
            role.position
        )
        .fetch_one(pool)
        .await?;

        Ok(row)
    }

    async fn delete(pool: &PgPool, id: i64) -> Result<DbRole, sqlx::Error> {
        let row = sqlx::query_as!(
            DbRole,
            r#"
            DELETE FROM roles
            WHERE id = $1
            RETURNING id, guild_id, mentionable, name, position
            "#,
            id
        )
        .fetch_one(pool)
        .await?;

        Ok(row)
    }


}