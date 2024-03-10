use axum::routing::get;
use axum::routing::post;
use axum::Router;

use social_axum::{
    create_post, create_user, establish_connection, get_all_posts, get_all_users, home,
};

#[tokio::main]

async fn main() {
    let pool = establish_connection().await.unwrap();

    let app = Router::new()
        .route("/user/create", post(create_user))
        .route("/user/all", get(get_all_users))
        .route("/post/create", post(create_post))
        .route("/", get(get_all_posts))
        .with_state(pool);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
