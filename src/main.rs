use axum::extract::State;
use axum::Router;
use axum::{routing::get, Json};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use social_axum::establish_connection;
use sqlx::PgPool;

#[tokio::main]

async fn main() {
    let pool = establish_connection().await.unwrap();

    let app = Router::new()
        .route("/", get(home))
        .route("/user/create", get(create_user))
        .route("/user/all", get(get_all_users))
        .with_state(pool);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn home() -> Json<Value> {
    let timestamp = chrono::prelude::Utc::now().to_string();

    Json(json!({"message": "Hello from Axum", "timestamp": timestamp}))
}

#[derive(serde::Serialize)]
struct UserInsert {
    name: &'static str,
    email: &'static str,
    password: &'static str,
}

async fn create_user(State(pool): State<PgPool>) -> Json<Value> {
    let data = UserInsert {
        name: "John Doe",
        email: "john@gmail.com",
        password: "password",
    };

    let _result = sqlx::query(
        "INSERT INTO users (name, email, password) VALUES ($1, $2, $3) RETURNING id, name, email",
    )
    .bind(data.name)
    .bind(data.email)
    .bind(data.password)
    .execute(&pool)
    .await
    .expect("Failed to insert user");

    Json(json!({"success": true, "user": data}))
}

#[derive(sqlx::FromRow, Serialize)]
struct UserSelect {
    id: i32,
    name: String,
    email: String,
    created_at: chrono::NaiveDateTime,
}

async fn get_all_users(State(pool): State<PgPool>) -> Json<Value> {
    let result: Vec<UserSelect> = sqlx::query_as("SELECT id, name, email, created_at FROM users")
        .fetch_all(&pool)
        .await
        .expect("Failed to fetch users");

    Json(json!({"success": true, "data": result}))
}
