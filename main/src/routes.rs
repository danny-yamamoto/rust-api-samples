use std::sync::Arc;
use axum::Json;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{Extension, extract::Query};
use cloud_storage::Client;
use sqlx::SqlitePool;
use model::UserQuery;

use crate::model::{StorageQuery, User, self, StorageResponse};

pub enum ApiResponse {
    UserResponse(Option<User>),
    StorageResponse(StorageResponse),
    ErrorResponse(String),
}

use axum::response::Response;

impl IntoResponse for ApiResponse {
    fn into_response(self) -> Response {
        match self {
            ApiResponse::UserResponse(user) => (StatusCode::OK, Json(user)).into_response(),
            ApiResponse::StorageResponse(storage) => (StatusCode::OK, Json(storage)).into_response(),
            ApiResponse::ErrorResponse(err) => (StatusCode::INTERNAL_SERVER_ERROR, Json(err)).into_response(),
        }
    }
}

pub async fn user_handler(Query(query):Query<UserQuery>, Extension(pool):Extension<Arc<SqlitePool>>) -> impl IntoResponse {
    let selected = query.user_id;
    match sqlx::query_as!(User, "select user_id, email_address, created_at, deleted, settings from users where user_id = ?", selected).fetch_optional(&*pool).await {
        Ok(user) => ApiResponse::UserResponse(user),
        Err(_) => ApiResponse::ErrorResponse("Internal Server Error".to_string()),
    }
}

pub async fn storage_handler(Query(query):Query<StorageQuery>) -> impl IntoResponse {
    let client = Client::default();
    match client.object().download(&query.bucket, &query.object).await {
        Ok(bytes) => ApiResponse::StorageResponse(StorageResponse { content: String::from_utf8_lossy(&bytes).to_string() }),
        Err(error) => ApiResponse::ErrorResponse(error.to_string()),
    }
}
