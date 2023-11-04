use axum::{
    routing::get,
    Router,
};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
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

// `/storage` パスのリクエストハンドラです。
async fn storage_handler() -> &'static str {
    "This is the storage page"
}
