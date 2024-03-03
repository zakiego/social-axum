use axum::{routing::get, Json, Router};
use diesel::prelude::*;
use serde_json::{json, Value};
use todo_axum::{create_post, establish_connection, models::Post};

#[tokio::main]
async fn main() {
    let connection = &mut establish_connection();

    let app = Router::new()
        .route("/", get(home))
        .route(
            "/posts",
            get(|| async {
                let connection = &mut establish_connection();
                get_all_posts(connection).await
            }),
        )
        .route(
            "/create",
            get(|| async {
                let connection = &mut establish_connection();
                create_post(connection, "Hello", "World").await
            }),
        );

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn home() -> Json<Value> {
    let timestamp = chrono::prelude::Utc::now().to_string();

    Json(json!({"message": "Hello from Axum", "timestamp": timestamp}))
}

async fn get_all_posts(conn: &mut PgConnection) -> Json<Value> {
    use todo_axum::schema::posts::dsl::*;
    let results = posts
        .select(Post::as_select())
        // .filter(published.eq(true))
        .limit(5)
        .load(conn);

    let results = match results {
        Ok(results) => results,
        Err(_) => return Json(json!({"data": "Error loading posts"})),
    };

    Json(json!({"data": results}))
}
