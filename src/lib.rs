use axum::Json;
use diesel::prelude::*;
use serde_json::json;
use std::env;

use dotenvy::dotenv;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("env var DATABASE_URL must be set");

    PgConnection::establish(&database_url)
        .unwrap_or_else(|err| panic!("Error connecting to {}: {}", database_url, err))
}

pub async fn create_post(
    conn: &mut PgConnection,
    title: &str,
    body: &str,
) -> Json<serde_json::Value> {
    use crate::models::Post;
    use schema::posts;

    let new_post = models::NewPost { title, body };

    let mut result = diesel::insert_into(posts::table)
        .values(&new_post)
        .returning(Post::as_returning())
        .get_result(conn)
        .expect("Error saving new post");

    Json(json!({ "data": &result }))
}

pub mod models;
pub mod schema;
