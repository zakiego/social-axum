use std::env;

use dotenvy::dotenv;
use sqlx::{postgres::PgPoolOptions, PgPool};

pub async fn establish_connection() -> Result<PgPool, sqlx::Error> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .unwrap_or_else(|_| panic!("Failed to create pool"));

    Ok(pool)
}
