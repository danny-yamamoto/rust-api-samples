use std::env;
use std::net::IpAddr;
use std::net::Ipv4Addr;
use std::net::SocketAddr;
use std::sync::Arc;

use axum::{
    extract::{Extension, Query},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, SqlitePool};

#[derive(Serialize, FromRow)]
struct User {
    user_id: i64,
    email_address: Option<String>,
    created_at: Option<i64>,
    deleted: Option<i64>,
    settings: Option<String>,
}

#[derive(Deserialize)]
struct UserQuery {
    user_id: i64,
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let key = "DATABASE_URL";
    let db_url = env::var(key).unwrap_or_else(|_| String::from("sqlite:./local.db"));
    println!("db_url: {}", db_url);

    let pool = SqlitePool::connect(&db_url).await.expect("Failed to create pool.");
    let shared_pool = Arc::new(pool);

    let app = Router::new()
        .route("/users", get(user_handler))
        .layer(Extension(shared_pool)); // Add the pool to the application state

    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn user_handler(
    Query(query): Query<UserQuery>,
    Extension(pool): Extension<Arc<SqlitePool>>,
) -> impl IntoResponse {
    let search_user_id = query.user_id;

    match sqlx::query_as!(User, "SELECT user_id, email_address, created_at, deleted, settings FROM users WHERE user_id = ?", search_user_id)
        .fetch_optional(&*pool)
        .await
    {
        Ok(user) => (StatusCode::OK, Json(user)),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(None::<User>)),
    }
}
