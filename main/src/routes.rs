use std::sync::Arc;

use axum::Json;

use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{Extension, extract::Query};
use cloud_storage::Client;
use sqlx::SqlitePool;
use model::UserQuery;
use model::User;

use crate::model::{self, StorageQuery, StorageResponse};

pub async fn user_handler(Query(query):Query<UserQuery>, Extension(pool):Extension<Arc<SqlitePool>>) -> impl IntoResponse {
    let selected = query.user_id;
    match sqlx::query_as!(User, "select user_id, email_address, created_at, deleted, settings from users where user_id = ?", selected).fetch_optional(&*pool).await {
        Ok(user) => (StatusCode::OK, Json(user)),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(None::<User>)),
    }
}

pub async fn storage_handler(Query(query):Query<StorageQuery>) -> impl IntoResponse {
    let bucket = query.bucket;
    let object = query.object;
    let client = Client::default();
    match client.object().download(&bucket, &object).await {
        Ok(_) => {
            (StatusCode::OK, axum::Json(None::<User>))
        },
        Err(_) => {
            (StatusCode::OK, axum::Json(None::<User>))
        },
    }
}
