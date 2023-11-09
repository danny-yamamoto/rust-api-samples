use std::env;

use dotenv::dotenv;
use sqlx::{SqlitePool, Row};

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    println!("hello");
    dotenv().ok();
    let key = "DATABASE_URL";
    let mut db_url = String::from("sqlite:./local.db");
    match env::var(key) {
        Ok(val) => {
            db_url = val;
            println!("success: {}", db_url);
        },
        Err(error) => {println!("failed: {}", error);},
    }
    println!("db_url: {}", db_url);
    let pool = SqlitePool::connect(&db_url).await?;

    let rows = sqlx::query("SELECT * FROM users")
    .fetch_all(&pool)
    .await?;
    // 取得したデータを出力
    for row in rows {
        let user_id: i32 = row.get("user_id");
        let email_address: String = row.get("email_address");
        let created_at: i64 = row.get("created_at");
        let deleted: i32 = row.get("deleted");
        let settings: String = row.get("settings");
        
        println!("User ID: {}, Email: {}, Created At: {}, Deleted: {}, Settings: {}",
                user_id, email_address, created_at, deleted, settings);
    }
    Ok(())
}
