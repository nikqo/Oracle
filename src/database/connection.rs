use std::env;

use sqlx::{
    Error as SqlxError,
    Pool,
    Postgres,
    postgres::PgPoolOptions,
};

pub async fn establish_connection() -> Result<Pool<Postgres>, SqlxError> {
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .map_err(|e| {
            eprintln!("Error connecting to database: {:?}", e);
            e
        })
}
