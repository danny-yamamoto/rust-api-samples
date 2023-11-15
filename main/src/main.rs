mod model;
mod routes;
use std::{env, sync::Arc, net::{SocketAddr, IpAddr, Ipv4Addr}};

use axum::{Router, routing::get, Extension};
use dotenv::dotenv;

use sqlx::SqlitePool;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let key = "DATABASE_URL";
    let db_url = env::var(key)
        .expect("key not fount.");
    let pool = SqlitePool::connect(&db_url)
        .await
        .expect("connection failed.");
    let shared_pool = Arc::new(pool);
    use routes::{user_handler, storage_handler};
    let app = Router::new()
        .route("/users", get(user_handler))
        .route("/storage", get(storage_handler))
        .layer(Extension(shared_pool));
    let sa = "SERVICE_ACCOUNT";
    match env::var(sa) {
        Ok(val) => println!("credentials path] {}", val),
        Err(error) => println!("credentials path not found. {}", error),
    }
    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
