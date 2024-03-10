use axum::extract::State;
use axum::Json;
use rand::Rng;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sqlx::PgPool;

use crate::{PostCreate, PostSelect, UserCreate, UserSelect};

pub fn home() -> Json<Value> {
    let timestamp = chrono::prelude::Utc::now().to_string();

    Json(json!({"message": "Hello from Axum", "timestamp": timestamp}))
}

fn create_token() -> String {
    // Generate a random token
    let token: String = rand::thread_rng()
        .sample_iter(&rand::distributions::Alphanumeric)
        .take(30)
        .map(char::from)
        .collect();
    token
}

pub async fn create_user(
    State(pool): State<PgPool>,
    Json(payload): Json<UserCreate>,
) -> Json<Value> {
    let access_token = create_token();

    let _result =
        sqlx::query("INSERT INTO users (username, password, access_token) VALUES ($1, $2, $3)")
            .bind(&payload.username)
            .bind(&payload.password)
            .bind(&access_token)
            .execute(&pool)
            .await
            .expect("Failed to insert user");

    Json(json!({
            "success": true,
            "message": "User created successfully, save the token!",
            "data" : {
                "username": &payload.username,
                "access_token": &access_token
            }
    }))
}

pub async fn get_all_users(State(pool): State<PgPool>) -> Json<Value> {
    let result: Vec<UserSelect> = sqlx::query_as("SELECT id, username, created_at FROM users")
        .fetch_all(&pool)
        .await
        .expect("Failed to fetch users");

    Json(json!({"success": true, "data": result}))
}

pub async fn create_post(
    State(pool): State<PgPool>,
    Json(payload): Json<PostCreate>,
) -> Json<Value> {
    // Check if the user exists and password is correct
    let user: UserSelect = sqlx::query_as(
        "SELECT id, username, created_at FROM users WHERE username = $1 AND password = $2",
    )
    .bind(&payload.username)
    .bind(&payload.password)
    .fetch_one(&pool)
    .await
    .expect("User not found or password is incorrect");

    let _result = sqlx::query("INSERT INTO posts (title, content, user_id) VALUES ($1, $2, $3)")
        .bind(&payload.title)
        .bind(&payload.content)
        .bind(&user.id)
        .execute(&pool)
        .await
        .expect("Failed to insert post");

    Json(
        json!({"success": true, "message": "Post created successfully", "data": {
            "title": &payload.title,
            "content": &payload.content,
            "username": &payload.username
        }}),
    )
}

pub async fn get_all_posts(State(pool): State<PgPool>) -> Json<Value> {
    let result: Vec<PostSelect> = sqlx::query_as(
        "SELECT 
            p.id, 
            p.title, 
            p.content, 
            p.created_at,
            u.username as username
        FROM posts p
        LEFT JOIN users u ON p.user_id = u.id
        ORDER BY p.created_at DESC
        ",
    )
    .fetch_all(&pool)
    .await
    .expect("Failed to fetch posts");

    Json(json!({"success": true, "data": result}))
}
