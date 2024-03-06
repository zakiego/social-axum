use std::env;

use axum::Router;
use axum::{routing::get, Json};
use serde_json::{json, Value};
use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    dotenvy::dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url.as_str())
        .await?;

    let row: (i64,) = sqlx::query_as("SELECT $1")
        .bind(150_i64)
        .fetch_one(&pool)
        .await?;

    println!("row: {:?}", row.0);

    assert_eq!(row.0, 150);

    Ok(())

    // let app = Router::new().route("/", get(home));

    // let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    // axum::serve(listener, app).await.unwrap();
}

async fn home() -> Json<Value> {
    let timestamp = chrono::prelude::Utc::now().to_string();

    Json(json!({"message": "Hello from Axum", "timestamp": timestamp}))
}
