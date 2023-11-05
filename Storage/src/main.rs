use axum::{
    routing::get,
    Router, extract::Query, http::StatusCode, response::IntoResponse, Json,
};
use cloud_storage::Client;
use serde::{Deserialize, Serialize};

use std::{net::SocketAddr, env};
use dotenv::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let key = "SERVICE_ACCOUNT";
    match env::var(key) {
        Ok(val) => println!("{}: {:?}", key, val),
        Err(e) => println!("couldn't interpret {}: {}", key, e)
    }
    let app = Router::new().route("/storage", get(storage_handler));
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    println!("Listening on http://{}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[derive(Deserialize)]
struct StorageQuery {
    bucket: String,
    object: String,
}

#[derive(Serialize)]
struct StorageResponse {
    content: String,
}

async fn storage_handler(Query(query): Query<StorageQuery>) -> impl IntoResponse {
    let client = Client::default();

    match client.object().download(&query.bucket, &query.object).await {
        Ok(bytes) => {
            let content = String::from_utf8_lossy(&bytes);
            let response = StorageResponse { content: content.to_string() };
            (StatusCode::OK, Json(response))
        },
        Err(error) => {
            let error_message = format!("Failed to read object: {}", error);
            let response = StorageResponse { content: error_message };
            (StatusCode::INTERNAL_SERVER_ERROR, Json(response))
        },
    }
}
