use std::sync::Arc;

use axum::Json;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{Extension, extract::Query};
use sqlx::SqlitePool;
use model::UserQuery;
use model::User;

use crate::model;

pub async fn user_handler(Query(query):Query<UserQuery>, Extension(pool):Extension<Arc<SqlitePool>>) -> impl IntoResponse {
    let selected = query.user_id;
    match sqlx::query_as!(User, "select user_id, email_address, created_at, deleted, settings from users where user_id = ?", selected)
        .fetch_optional(&*pool)
        .await {
        Ok(user) => (StatusCode::OK, Json(user)),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(None::<User>)),
    }
}
