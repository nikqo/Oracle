use serenity::async_trait;
use serenity::model::gateway::Ready;
use serenity::prelude::*;

pub struct Handler {
    pub pool: sqlx::Pool<sqlx::Postgres>,
}

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _ctx: Context, ready: Ready) {
        log::info!("{} is connected!", ready.user.name)
    }
}