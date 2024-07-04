// External Imports //
use serenity::prelude::*;
use std::env;

use log::LevelFilter;

// Internal Modules & Imports //
pub mod database;
pub mod event_handler;
pub mod logger;

use event_handler::Handler;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();

    let mut logger = logger::Logger::new(LevelFilter::Trace);
    if let Err(e) = logger.start() {
        eprintln!("Error starting logger: {:?}", e);
        return Err("Failed to start logger".into());
    }

    let token = env::var("BOT_TOKEN").map_err(|_| "BOT_TOKEN environment variable not found")?;
    let intents = GatewayIntents::all();

    let pool = database::connection::establish_connection()
        .await
        .map_err(|e| format!("Error connecting to database: {:?}", e))?;

    let handler = Handler { pool };

    let mut client = Client::builder(token, intents)
        .event_handler(handler)
        .await
        .map_err(|e| format!("Error creating client: {:?}", e))?;

    if let Err(why) = client.start().await {
        eprintln!("Client error: {:?}", why);
        return Err("Failed to start client".into());
    }

    Ok(())
}
