use sqlx::FromRow;

use serenity::model::{
    user::User,
    guild::Guild,
};

use serde_json::{Value, to_string, to_value};

#[derive(Debug, FromRow)]
pub struct DbUser {
    pub id: i64,
    pub name: String,
    pub discriminator: Option<i16>,
    pub global_name: Option<String>,
    pub avatar: Option<String>,
    pub bot: bool,
    pub system: bool,
    pub mfa_enabled: bool, 
    pub locale: Option<String>,
    pub flags: i64,
    pub premium_type: String,
    pub public_flags: Option<i64>,
}

impl From<User> for DbUser {
    fn from(user: User) -> Self {
        DbUser {
            id: user.id.into(), 
            name: user.name,
            discriminator: user.discriminator.map(|d| d.get() as i16),
            global_name: user.global_name,
            avatar: user.avatar.map(|a| to_string(&a).unwrap_or_default()),
            bot: user.bot,
            system: user.system,
            mfa_enabled: user.mfa_enabled,
            locale: user.locale,
            flags: user.flags.bits() as i64,
            premium_type: to_string(&user.premium_type).unwrap_or_default(),
            public_flags: user.public_flags.map(|flags| flags.bits() as i64),
        }
    }
}

#[derive(Debug, FromRow)]
pub struct DbGuild {
    pub id: i64,
    pub name: String,
    pub icon: Option<String>,
    pub icon_hash: Option<String>,
    pub splash: Option<String>,
    pub discovery_splash: Option<String>,
    pub owner_id: i64, // foreign key to users
    pub afk_metadata: Option<Value>, // needs to be a json
    pub widget_enabled: Option<bool>,
    pub widget_channel_id: Option<i64>,
    pub verification_level: Option<String>, // enum
    pub default_message_notifications: Option<String>, // enum
    pub explicit_content_filter: Option<String>, // enum
    pub roles: Value, // a hashmap, roleid & role. needs to be a json
    pub emojis: Value, // a hashmap, emojiid & emoji. needs to be a json
}

impl From<Guild> for DbGuild {
    fn from(guild: Guild) -> Self {
        DbGuild {
            id: guild.id.into(),
            name: guild.name,
            icon: guild.icon.map(|i| to_string(&i).unwrap_or_default()),
            icon_hash: guild.icon_hash.map(|i| to_string(&i).unwrap_or_default()),
            splash: guild.splash.map(|s| to_string(&s).unwrap_or_default()),
            discovery_splash: guild.discovery_splash.map(|d| to_string(&d).unwrap_or_default()),
            owner_id: guild.owner_id.into(),
            afk_metadata: Some(to_value(&guild.afk_metadata).unwrap()),
            widget_enabled: guild.widget_enabled,
            widget_channel_id: guild.widget_channel_id.map(|c| c.into()),
            verification_level: Some(to_string(&guild.verification_level).unwrap_or_default()),
            default_message_notifications: Some(to_string(&guild.default_message_notifications).unwrap_or_default()),
            explicit_content_filter: Some(to_string(&guild.explicit_content_filter).unwrap_or_default()),
            roles: to_value(&guild.roles).unwrap(),
            emojis: to_value(&guild.emojis).unwrap(),
        }
    }
}
