use serenity::prelude::*;
use serenity::async_trait;
use serenity::model::gateway::Ready;

// use database::model::DbUser;

pub mod database;

struct Handler {
    pool: sqlx::Pool<sqlx::Postgres>,
}


#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        // Check if the database connection is working
        match sqlx::query("SELECT 1")
            .fetch_one(&self.pool)
            .await {
                Ok(_) => println!("Database connection is working"),
                Err(e) => eprintln!("Error connecting to database: {:?}", e),
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
        .event_handler(Handler { pool })
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}   

