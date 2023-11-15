use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct User {
    pub user_id: i64,
    pub email_address: Option<String>,
    pub created_at: Option<i64>,
    pub deleted: Option<i64>,
    pub settings: Option<String>,
}

#[derive(Deserialize)]
pub struct UserQuery {
    pub user_id: i64,
}

#[derive(Deserialize)]
pub struct StorageQuery {
    pub bucket: String,
    pub object: String,
}

#[derive(Serialize)]
pub struct  StorageResponse {
    pub content: String,
}
