use sqlx::postgres::PgPoolOptions;
use std::env;

pub async fn establish_connection() -> Result<sqlx::Pool<sqlx::Postgres>, sqlx::Error> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await;

    match pool {
        Ok(pool) => Ok(pool),
        Err(e) => {
            eprintln!("Error connecting to database: {:?}", e);
            Err(e)
        }
    }
}

