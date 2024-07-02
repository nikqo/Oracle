use serenity::prelude::*;
use serenity::async_trait;
use serenity::model::gateway::Ready;

use database::model::DbUser;

pub mod database;

struct Handler {
    pool: sqlx::Pool<sqlx::Postgres>,
}


#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }

    async fn cache_ready(&self, ctx: Context, _guilds: Vec<serenity::model::id::GuildId>) {
        println!("Cache is ready!");
        let users = ctx.cache.users();

        for user_ref in users.iter() {
            let user = user_ref.value();
            let db_user: DbUser = user.into();

            let result = database::queries::insert_user(&self.pool, &db_user).await;
            match result {
                Ok(_) => println!("Inserted user {} into database", user.name),
                Err(e) => eprintln!("Error inserting user {} into database: {:?}", user.name, e),
            }
        }

        

    }
}

#[tokio::main]
async fn main() {

    dotenv::dotenv().ok();

    let token = std::env::var("BOT_TOKEN")
        .expect("Expected a token in the environment");
    let intents = GatewayIntents::all();
    let pool = database::connection::establish_connection().await.expect("Error connecting to database");

    let mut client = Client::builder(token, intents)
        .event_handler(Handler{pool: pool})
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}   

