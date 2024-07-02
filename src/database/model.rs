use serde::{Deserialize, Serialize};
use serenity::model::user::User;
// use serenity::model::id::UserId;
use core::num::NonZeroU16;
use serenity::model::colour::Colour;
// use serenity::model::misc::ImageHash;


#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct DbUser {
    pub id: i64,
    pub name: String,
    pub discriminator: Option<NonZeroU16>,
    pub global_name: Option<String>,
    pub avatar: Option<String>,
    pub bot: bool,
    pub banner: Option<String>,
    pub accent_colour: Option<Colour>,
}

impl From<&User> for DbUser {
    fn from(user: &User) -> Self {
        Self {
            id: user.id.into(),
            name: user.name.clone(),
            discriminator: user.discriminator,
            global_name: user.global_name.clone(),
            avatar: user.avatar.as_ref().map(|hash| hash.to_string()),
            bot: user.bot,
            banner: user.banner.as_ref().map(|hash| hash.to_string()),
            accent_colour: user.accent_colour,
        }
    }
}