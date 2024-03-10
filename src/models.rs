use serde::{Deserialize, Serialize};
use sqlx::types::Uuid;

#[derive(sqlx::FromRow, Serialize)]
pub struct UserSelect {
    pub id: Uuid,
    username: String,
    created_at: chrono::NaiveDateTime,
}

#[derive(Deserialize, Serialize)]
pub struct UserCreate {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize, Serialize)]
pub struct PostCreate {
    pub title: String,
    pub content: String,
    pub username: String,
    pub password: String,
}

#[derive(sqlx::FromRow, Serialize)]
pub struct PostSelect {
    pub id: Uuid,
    pub title: String,
    pub content: String,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub username: String,
}
