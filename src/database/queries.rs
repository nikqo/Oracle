use sqlx::{PgPool, Error};

use crate::database::model::{DbUser, DbGuild};
use log::{info, error};
trait _CrudOperations {
    async fn create(&self, pool: &PgPool) -> Result<(), Error>;
    async fn read(&self, pool: &PgPool) -> Result<Option<Self>, Error> where Self: Sized;
    async fn update(&self, pool: &PgPool) -> Result<(), Error>;
    async fn delete(&self, pool: &PgPool) -> Result<(), Error>;
}

impl _CrudOperations for DbUser {
    async fn create(&self, pool: &PgPool) -> Result<(), sqlx::Error> {
        match sqlx::query!(
            r#"
            INSERT INTO users (id, name, discriminator, global_name, avatar, bot, system, mfa_enabled, locale, flags, premium_type, public_flags)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
            "#,
            self.id,
            self.name,
            self.discriminator,
            self.global_name,
            self.avatar,
            self.bot,
            self.system,
            self.mfa_enabled,
            self.locale,
            self.flags,
            self.premium_type,
            self.public_flags
        )
        .execute(pool)
        .await {
            Ok(_) => info!("User {} inserted into the database", self.name),
            Err(e) => error!("Error inserting user {} into the database: {}", self.name, e),
        }

        Ok(())
    }

    async fn read(&self, pool: &PgPool) -> Result<Option<Self>, sqlx::Error> {
        match sqlx::query_as!(
            DbUser,
            r#"
            SELECT * FROM users WHERE id = $1
            "#,
            self.id
        )
        .fetch_optional(pool)
        .await {
            Ok(user) => Ok(user),
            Err(e) => {
                error!("Error reading user {} from the database: {}", self.id, e);
                Ok(None)
            }
        }
    }

    async fn update(&self, pool: &PgPool) -> Result<(), sqlx::Error> {
        match sqlx::query!(
            r#"
            UPDATE users
            SET name = $2, discriminator = $3, global_name = $4, avatar = $5, bot = $6, system = $7, mfa_enabled = $8, locale = $9, flags = $10, premium_type = $11, public_flags = $12
            WHERE id = $1
            "#,
            self.id,
            self.name,
            self.discriminator,
            self.global_name,
            self.avatar,
            self.bot,
            self.system,
            self.mfa_enabled,
            self.locale,
            self.flags,
            self.premium_type,
            self.public_flags
        )
        .execute(pool)
        .await {
            Ok(_) => info!("User {} updated in the database", self.name),
            Err(e) => error!("Error updating user {} in the database: {}", self.name, e),
        }

        Ok(())
    }

    async fn delete(&self, pool: &PgPool) -> Result<(), sqlx::Error> {
        match sqlx::query!(
            r#"
            DELETE FROM users WHERE id = $1
            "#,
            self.id
        )
        .execute(pool)
        .await {
            Ok(_) => info!("User {} deleted from the database", self.name),
            Err(e) => error!("Error deleting user {} from the database: {}", self.name, e),
        }

        Ok(())
    }
}


impl _CrudOperations for DbGuild {
    async fn create(&self, pool: &PgPool) -> Result<(), sqlx::Error> {
        match sqlx::query!(
            r#"
            INSERT INTO guilds (id, name, icon, icon_hash, splash, discovery_splash, owner_id, afk_metadata, widget_enabled, widget_channel_id, verification_level, default_message_notifications, explicit_content_filter, roles, emojis)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15)
            "#,
            self.id,
            self.name,
            self.icon,
            self.icon_hash,
            self.splash,
            self.discovery_splash,
            self.owner_id,
            self.afk_metadata,
            self.widget_enabled,
            self.widget_channel_id,
            self.verification_level,
            self.default_message_notifications,
            self.explicit_content_filter,
            self.roles,
            self.emojis
        )
        .execute(pool)
        .await {
            Ok(_) => info!("Guild {} inserted into the database", self.name),
            Err(e) => error!("Error inserting guild {} into the database: {}", self.name, e),
        }

        Ok(())
    }

    async fn read(&self, pool: &PgPool) -> Result<Option<Self>, sqlx::Error> {
        match sqlx::query_as!(
            DbGuild,
            r#"
            SELECT * FROM guilds WHERE id = $1
            "#,
            self.id
        )
        .fetch_optional(pool)
        .await {
            Ok(guild) => Ok(guild),
            Err(e) => {
                error!("Error reading guild {} from the database: {}", self.id, e);
                Ok(None)
            }
        }
    }

    async fn update(&self, pool: &PgPool) -> Result<(), sqlx::Error> {
        match sqlx::query!(
            r#"
            UPDATE guilds
            SET name = $2, icon = $3, icon_hash = $4, splash = $5, discovery_splash = $6, owner_id = $7, afk_metadata = $8, widget_enabled = $9, widget_channel_id = $10, verification_level = $11, default_message_notifications = $12, explicit_content_filter = $13, roles = $14, emojis = $15
            WHERE id = $1
            "#,
            self.id,
            self.name,
            self.icon,
            self.icon_hash,
            self.splash,
            self.discovery_splash,
            self.owner_id,
            self.afk_metadata,
            self.widget_enabled,
            self.widget_channel_id,
            self.verification_level,
            self.default_message_notifications,
            self.explicit_content_filter,
            self.roles,
            self.emojis
        )
        .execute(pool)
        .await {
            Ok(_) => info!("Guild {} updated in the database", self.name),
            Err(e) => error!("Error updating guild {} in the database: {}", self.name, e),
        }

        Ok(())
    }

    async fn delete(&self, pool: &PgPool) -> Result<(), sqlx::Error> {
        match sqlx::query!(
            r#"
            DELETE FROM guilds WHERE id = $1
            "#,
            self.id
        )
        .execute(pool)
        .await {
            Ok(_) => info!("Guild {} deleted from the database", self.name),
            Err(e) => error!("Error deleting guild {} from the database: {}", self.name, e),
        }

        Ok(())
    }
}
