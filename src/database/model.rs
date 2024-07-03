use serde::{Deserialize, Serialize};
use serenity::all::GuildChannel;
use serenity::model::user::User;
use serenity::model::guild::Guild;
use serenity::model::guild::Role;
use chrono::NaiveDateTime;


#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct DbUser {
    pub id: i64,
    pub name: String,
    pub discriminator: Option<i32>,
    pub global_name: Option<String>,
    pub avatar: Option<String>,
    pub bot: bool,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct DbGuild {
    pub id: i64,
    pub name: String,
    pub icon: Option<String>,
    pub icon_hash: Option<String>,
    pub splash: Option<String>,
    pub owner_id: i64,
}

// a lot more can be added, just adding basics for now.
#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct DbGuildChannel {
    pub id: i64,
    pub guild_id: i64,
    pub name: String,
    pub position: i32,
    pub nsfw: bool,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct DbMessage {
    pub id: i64,
    pub channel_id: i64,
    pub author: i64,
    pub content: String,
    pub timestamp: NaiveDateTime,
    pub pinned: bool,
    pub guild_id: i64,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct DbRole {
    pub id: i64,
    pub guild_id: i64,
    // pub color: i64,( implement later, cba to do colors yet )
    pub mentionable: bool,
    pub name: String,
    // pub permissions: idk yet, 
    pub position: i32,
}

// Implementations from Serenity models to Database models

impl From<&User> for DbUser {
    fn from(user: &User) -> Self {
        Self {
            id: user.id.into(),
            name: user.name.clone(),
            discriminator: user.discriminator.map(|d| d.get() as i32),
            global_name: user.global_name.clone(),
            avatar: user.avatar.as_ref().map(|hash| hash.to_string()),
            bot: user.bot,
        }
    }
}

impl From<&Guild> for DbGuild {
    fn from(guild: &Guild) -> Self {
        Self {
            id: guild.id.into(),
            name: guild.name.clone(),
            icon: guild.icon.as_ref().map(|hash| hash.to_string()),
            icon_hash: guild.icon_hash.as_ref().map(|hash| hash.to_string()),
            splash: guild.splash.as_ref().map(|hash| hash.to_string()),
            owner_id: guild.owner_id.into(),
        }
    }
}

impl From<&GuildChannel> for DbGuildChannel {
    fn from(channel: &GuildChannel) -> Self {
        Self {
            id: channel.id.into(),
            guild_id: channel.guild_id.into(),
            name: channel.name.clone(),
            position: channel.position as i32,
            nsfw: channel.nsfw,
        }
    }
}

impl From<&serenity::model::channel::Message> for DbMessage {
    fn from(message: &serenity::model::channel::Message) -> Self {
        Self {
            id: message.id.into(),
            channel_id: message.channel_id.into(),
            author: message.author.id.into(),
            content: message.content.clone(),
            timestamp: message.timestamp.naive_utc(),
            pinned: message.pinned,
            guild_id: message.guild_id.unwrap().into(),
        }
    }
}

impl From<&Role> for DbRole {
    fn from(role: &Role) -> Self {
        Self {
            id: role.id.into(),
            guild_id: role.guild_id.into(),
            mentionable: role.mentionable,
            name: role.name.clone(),
            position: role.position as i32,
        }
    }
}