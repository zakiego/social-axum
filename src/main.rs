use axum::{routing::get, Json, Router};
use serde_json::{json, Value};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(home));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn home() -> Json<Value> {
    let timestamp = chrono::prelude::Utc::now().to_string();

    Json(json!({"message": "Hello from Axum", "timestamp": timestamp}))
}
