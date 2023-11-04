use axum::{
    routing::get,
    Router, extract::Query, http::StatusCode, response::IntoResponse,
};
use cloud_storage::Client;
use serde::Deserialize;
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
    // アプリケーションのルーターを構築します。
    let app = Router::new().route("/storage", get(storage_handler));

    // サーバーのアドレスを設定します。
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));

    println!("Listening on http://{}", addr);

    // サーバーを起動します。
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

// `/storage` パスのリクエストハンドラです。
async fn storage_handler(Query(query): Query<StorageQuery>) -> impl IntoResponse {
    let client = Client::default();

    // 指定されたバケットとオブジェクトでデータを読み込みます。
    match client.object().download(&query.bucket, &query.object).await {
        Ok(bytes) => (StatusCode::OK, bytes),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Failed to read object".into()),
    }
}
