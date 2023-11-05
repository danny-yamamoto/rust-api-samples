---
title: "Rust vs. Go: Building & Comparing REST APIs for Cloud Storage"
emoji: "🚀"
type: "tech" # tech: 技術記事 / idea: アイデア
topics: ["rust", "go", "googlecloudstorage"]
published: false
---
Rust の Production での実装について、他社の利用状況を見ると、Web App に導入していけそう。

あとの問題は、team や会社の skill set をどうするか。Ops できるか。

Cloud Functions など、runtime に依存するものはまだ動かせない。Cloud Run など container service では動かせる。

microservices 関連で考えると、Otel への対応が気になる。dependencies に追加することで実装は可能かもしれない。この辺りは別途検証したい。

まだまだ、ecosystem は不足している部分はある。今後の Rust ecosystem に期待。

Rust の Code を流用する場合は、dependencies のチェックが必要。変なものが紛れていないか。

## 課題
1. state を扱う API にしたい。
1. 業務で使うような 外部の API を call するものが良い。

上記の理由で、Cloud Storage を操作する API を Rust と Go で実装し、比較する。

Rust/Go の API をスラスラ書けるようになるように、繰り返し書いて身につける。

コードはこちら。

https://github.com/danny-yamamoto/rust-api-samples

## Rust
```rust
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
```

### GPT-4 の解説
> このRustコードは、Axumというウェブフレームワークとcloud_storageというクレート（ライブラリ）を使って、単純なHTTPサーバーを実装しています。サーバーはGoogle Cloud Storageのバケットからオブジェクトをダウンロードするエンドポイントを提供します。以下にコードの主要な部分を説明します。
> * main関数
#[tokio::main] アトリビュートは非同期関数を開始するためのトキオランタイムのエントリポイントを指定します。
dotenv().ok(); で .env ファイルから環境変数をロードします。
環境変数SERVICE_ACCOUNTを読み込み、存在すればその値を、存在しなければエラーを表示します。
Router::new() で新しいルーターを作成し、/storageパスにstorage_handler関数をGETリクエストのハンドラとして結びつけます。
axum::Server::bind(&addr) で指定したアドレスとポート番号でサーバーを起動します。
> * storage_handler関数
storage_handler 関数はStorageQueryをクエリパラメータとして受け取り、非同期でGoogle Cloud Storageから指定されたバケットとオブジェクトをダウンロードします。
ダウンロードが成功すると、取得したデータをUTF-8でデコードし、StorageResponseをJSONとしてレスポンスとして返します。
エラーが発生した場合は、エラーメッセージを含むStorageResponseをJSONとして返し、HTTPステータスコードとしてはINTERNAL_SERVER_ERRORを設定します。
> 
> このコードを実行するには、事前にRustの環境設定が必要で、必要な依存関係をCargo.tomlに追加し、Google Cloud Storageへの認証情報が必要です（これは環境変数経由で渡されます）。また、適切な.envファイルまたは環境変数の設定も必要です。

## Go
```golang
```
