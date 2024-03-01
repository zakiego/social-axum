use axum::{routing::get, Json, Router};
use serde_json::{json, Value};
#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(home));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn home() -> Json<Value> {
    Json(json!({"message": "Hello, World!"}))
}
