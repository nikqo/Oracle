use serde::{Deserialize, Serialize};
use serenity::model::user::User;
use serenity::model::guild::Guild;
// use serenity::model::id::UserId;
use core::num::NonZeroU16;
// use serenity::model::misc::ImageHash;


#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct DbUser {
    pub id: i64,
    pub name: String,
    pub discriminator: Option<NonZeroU16>,
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
    pub owner_name: String,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct DbChannel {

}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct DbMessage {

}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct DbRole {

}

pub struct DbCategory {

}

// Implementations from Serenity models to Database models

impl From<&User> for DbUser {
    fn from(user: &User) -> Self {
        Self {
            id: user.id.into(),
            name: user.name.clone(),
            discriminator: user.discriminator,
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
            owner_name: "Test".to_string(), 
        }
    }
}