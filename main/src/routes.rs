use std::sync::Arc;
use axum::Json;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{Extension, extract::Query};
use cloud_storage::Client;
use sqlx::SqlitePool;
use model::UserQuery;
use axum::response::Response;

use crate::model::{StorageQuery, User, self, StorageResponse};

pub enum ApiResponse {
    UserResponse(Option<User>),
    StorageResponse(StorageResponse),
    ErrorResponse(String),
}

impl IntoResponse for ApiResponse {
    fn into_response(self) -> Response {
        match self {
            ApiResponse::UserResponse(user) => (StatusCode::OK, Json(user)).into_response(),
            ApiResponse::StorageResponse(storage) => (StatusCode::OK, Json(storage)).into_response(),
            ApiResponse::ErrorResponse(err) => (StatusCode::INTERNAL_SERVER_ERROR, Json(err)).into_response(),
        }
    }
}

pub async fn user_handler(Query(query):Query<UserQuery>, Extension(user_service):Extension<Arc<UserService>>) -> impl IntoResponse {
    match user_service.fetch_user(query.user_id).await {
        Ok(user) => ApiResponse::UserResponse(user),
        Err(_) => ApiResponse::ErrorResponse("Internal Server Error".to_string()),
    }
}

pub struct UserService {
    pool: SqlitePool,
}

impl UserService {
    pub fn new(pool: SqlitePool) -> Self {
        UserService { pool }
    }

    pub async fn fetch_user(&self, user_id: i64) -> Result<Option<User>, sqlx::Error> {
        sqlx::query_as!(User, "SELECT user_id, email_address, created_at, deleted, settings FROM users WHERE user_id = ?", user_id).fetch_optional(&self.pool).await
    }
}

pub async fn storage_handler(Query(query):Query<StorageQuery>, Extension(storage_service):Extension<Arc<StorageService>>) -> impl IntoResponse {
    match storage_service.download_content(&query).await {
        Ok(content) => ApiResponse::StorageResponse(content),
        Err(error) => ApiResponse::ErrorResponse(error.to_string()),
    }
}

pub struct StorageService;

impl StorageService {
    pub fn new() -> Self {
        StorageService {}
    }

    pub async fn download_content(&self, query: &StorageQuery) -> Result<StorageResponse, String> {
        let client = Client::default();
        match client.object().download(&query.bucket, &query.object).await {
            Ok(bytes) => Ok(StorageResponse { content: String::from_utf8_lossy(&bytes).to_string() }),
            Err(error) => Err(error.to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_api_response_users() {
        let user = User {
            user_id: 9999,
            email_address: Some("hoge@example.com".to_string()),
            created_at: Some(0),
            deleted: Some(0),
            settings: Some("option".to_string()),
        };
        let response = ApiResponse::UserResponse(Some(user)).into_response();
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn err_api_response_users() {
        let user = "error".to_string();
        let response = ApiResponse::ErrorResponse(user).into_response();
        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

}

#[cfg(test)]
mod users_service_tests {
    use serde::{Serialize, Deserialize};
    use sqlx::SqlitePool;
    use crate::routes::UserService;

    #[tokio::test]
    async fn test_fetch_users() {
        let pool = SqlitePool::connect("sqlite:./unit_test.db").await.expect("Failed to connect to database.");
        let service = UserService::new(pool);
        let user_id = 10000;
        let result = service.fetch_user(user_id).await;
        assert!(result.is_ok());
        let user = result.unwrap();
        assert!(user.is_some());
        let row = user.unwrap();
        assert_eq!(row.email_address, Some("marc@example.com".to_string()));
    }

    #[derive(Debug, Serialize, Deserialize)]
    struct TestCase {
		name: String,
		user_id: i64,
		expected_json: String,
    }

    #[tokio::test]
    async fn tdt_fetch_users() {
        let pool = SqlitePool::connect("sqlite:./unit_test.db").await.expect("Failed to connect to database.");
        let service = UserService::new(pool);
        let tests = vec![
            TestCase { name: "Normal pattern a".to_string(), user_id: 10000, expected_json: "{\"user_id\":10000,\"email_address\":\"marc@example.com\",\"created_at\":0,\"deleted\":1,\"settings\":\"\"}".to_string() },
            TestCase { name: "Normal pattern b".to_string(), user_id: 100, expected_json: "{\"user_id\":100,\"email_address\":\"alex@example.com\",\"created_at\":1,\"deleted\":0,\"settings\":\"\"}".to_string() }
        ];

        for tc in tests {
            let user_id = tc.user_id;
            let result = service.fetch_user(user_id).await;
            assert!(result.is_ok());
            let user_data = result.unwrap();
            let user_json = serde_json::to_string(&user_data).expect("error");
            assert_eq!(user_json, tc.expected_json, "Failed: {}", tc.name);
        }
    }
}
