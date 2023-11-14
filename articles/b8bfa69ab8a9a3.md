---
title: "Rust vs. Go: Building & Comparing REST APIs for Cloud Storage"
emoji: "🚀"
type: "tech" # tech: 技術記事 / idea: アイデア
topics: ["rust", "go", "googlecloud", "gcs", "devcontainer"]
published: false
---
Rust の Production での実装について、他社の利用状況を見ると、web app に導入していけそう。2023年11月時点。

Rust の syntax になれるため、Rust を Go に書き直す。双方を関連付けて覚えていく。

Rust の課題は、組織。team や会社の skill set をどうするか。まだ、自分の周りでは、キャズムを超えていないため、ops が問題になり得る。

また、`Cloud Functions` など、言語に依存するものはまだ動かせない。`Cloud Run` など container service では動かせる。

microservices 関連で考えると、`OpenTelemetry`（Otel）への対応が気になる。`dependencies` に追加することで実装は可能かもしれない。この辺りは別途検証したい。

一方で、まだまだ、ecosystem は不足している部分はある。今後の Rust ecosystem の進化に期待。

気をつける点として、`dependencies` のチェックが必要。変なものが紛れていないか。

## 課題
- state を扱う API にしたい。
- 業務で使うような 外部の API を call するものが良い。

上記の要件に合致するものとして、今回は `Cloud Storage` を操作する API を Rust と Go で実装し、比較する。

Rust/Go の API をスラスラ書けるようになるように、繰り返し書いて身につける。特に、HTTP Server は、どこでも使うため。

コードはこちら。

https://github.com/danny-yamamoto/rust-api-samples

https://github.com/danny-yamamoto/go-api-samples

## Rust Web Server: /storage
実装の手順
1. Requestの構造体 `StorageQuery` を書く
1. Response の構造体 `StorageResponse` を書く
1. handler を書く。download と `IntoResponse` 以外。
1. `main` を書く
1. handler の download と `IntoResponse` を書く。

```toml:Cargo.toml
[package]
name = "storage"
version = "0.1.0"
edition = "2021"

[dependencies]
rand = "0.8.5"
axum = "0.6.20"
tokio = { version = "1.33.0", features = ["full"] }
cloud-storage = "0.11.1"
serde = { version = "1.0.190", features = ["derive"] }
dotenv = "0.15.0"
serde_json = "1.0.108"
```

```rust:main.rs
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

## Rust to Go Conversion
実装の手順
1. `go get google.golang.org/api/option` `go get google.golang.org/api/storage/v1`
1. 構造体 `StorageQuery` を書く
1. 構造体 `StorageResponse` を書く
1. handler `storageHandler` を書く
1. Handler を書く
1. Handler をインスタンス化
1. `main` を書く

```go
package main

import (
	"context"
	"encoding/json"
	"fmt"
	"io"
	"log"
	"net/http"
	"os"

	"google.golang.org/api/option"
	"google.golang.org/api/storage/v1"
)

type StorageQuery struct {
	Bucket string `json:"bucket"`
	Object string `json:"object"`
}

type StorageResponse struct {
	Content string `json:"content"`
}

func (h *Handler) storageHandler(w http.ResponseWriter, r *http.Request) {
	bucket := r.URL.Query().Get("bucket")
	object := r.URL.Query().Get("object")

	rc, err := h.client.Objects.Get(bucket, object).Download()
	if err != nil {
		respondWithError(w, fmt.Sprintf("Failed to read object: %v", err))
		return
	}
	defer rc.Body.Close()

	data, err := io.ReadAll(rc.Body)
	if err != nil {
		respondWithError(w, fmt.Sprintf("Failed to read object data: %v", err))
		return
	}

	respondWithJSON(w, http.StatusOK, StorageResponse{Content: string(data)})
}

func respondWithJSON(w http.ResponseWriter, statusCode int, payload interface{}) {
	response, _ := json.Marshal(payload)
	w.Header().Set("Content-Type", "application/json")
	w.WriteHeader(statusCode)
	w.Write(response)
}

func respondWithError(w http.ResponseWriter, message string) {
	respondWithJSON(w, http.StatusInternalServerError, ErrorResponse{Error: message})
}

type Handler struct {
	client *storage.Service
}

func NewHandler(client *storage.Service) *Handler {
	return &Handler{client: client}
}

func main() {
	ctx := context.Background()
	client, err := storage.NewService(ctx, option.WithCredentialsFile(os.Getenv("GOOGLE_APPLICATION_CREDENTIALS")))
	if err != nil {
		fmt.Printf("Failed to create client: %s", err)
		return
	}

	handler := NewHandler(client)
	http.HandleFunc("/storage", handler.storageHandler)

	port := "8080"
	if fromEnv := os.Getenv("PORT"); fromEnv != "" {
		port = fromEnv
	}

	addr := fmt.Sprintf("0.0.0.0:%s", port)
	fmt.Printf("Listening on http://%s\n", addr)

	log.Fatal(http.ListenAndServe(addr, nil))
}
```
