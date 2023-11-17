mod model;
mod routes;
use std::{env, sync::Arc, net::{SocketAddr, IpAddr, Ipv4Addr}};

use axum::{Router, routing::get, Extension};
use dotenv::dotenv;

use sqlx::SqlitePool;

use crate::routes::{UserService, StorageService};

#[tokio::main]
async fn main() {
    dotenv().ok();
    let key = "DATABASE_URL";
    let db_url = env::var(key)
        .expect("Environment variable DATABASE_URL is not set.");
    let pool = SqlitePool::connect(&db_url)
        .await
        .expect("Failed to connect to database.");

    let sa = "SERVICE_ACCOUNT";
    match env::var(sa) {
        Ok(val) => println!("Environment Variable SERVICE_ACCOUNT is {}", val),
        Err(error) => println!("Environment Variable SERVICE_ACCOUNT is not set. {}", error),
    }
    
    use routes::{user_handler, storage_handler};
    let user_service = Arc::new(UserService::new(pool));
    let storage_service = Arc::new(StorageService::new());
    let app = Router::new()
        .route("/users", get(user_handler).layer(Extension(user_service)))
        .route("/storage", get(storage_handler).layer(Extension(storage_service)));
    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
