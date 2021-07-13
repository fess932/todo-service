use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions, SqliteRow};
use sqlx::Row;
use sqlx::SqlitePool;
use std::str::FromStr;

use crate::buisness;

const CREATE_TABLE: &str = r#"CREATE TABLE IF NOT EXISTS users(id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL, name TEXT NOT NULL)"#;

pub struct SqliteStore(SqlitePool);

#[async_trait]
impl buisness::Store for SqliteStore {
    async fn store(&self, name: String) {
        println!("store impl name: {}", name);

        sqlx::query(r#"INSERT INTO users (name) VALUES (?1)"#)
            .bind(name)
            .execute(&self.0)
            .await
            .expect("cannot add users");
    }

    async fn get_users(&self) -> Vec<buisness::User> {
        println!("get users from db");

        let users = sqlx::query("SELECT * From users")
            .map(|row: SqliteRow| {
                let name = row.try_get("name").expect("not found name");
                buisness::User { name: name }
            })
            .fetch_all(&self.0)
            .await
            .expect("cannot fetch users from db");

        println!("get users from db");

        return users;
    }
}

pub async fn new_store() -> Result<Box<dyn buisness::Store + Send + Sync>, sqlx::Error> {
    let pool = prepare().await.expect("wrong prepared db");

    let store = SqliteStore(pool);

    Ok(Box::new(store))
}

async fn prepare() -> Result<SqlitePool, sqlx::Error> {
    let conn = SqliteConnectOptions::from_str("sqlite:db.sqlite3")?.create_if_missing(true);

    let pool = SqlitePoolOptions::new()
        .connect_with(conn)
        .await
        .expect("pool failed");

    sqlx::query(CREATE_TABLE)
        .execute(&pool)
        .await
        .expect("create table users failed");

    let name = String::from("ivan");

    sqlx::query(r#"INSERT INTO users (name) VALUES (?1)"#)
        .bind(name)
        .execute(&pool)
        .await
        .expect("not added user to user table");

    Ok(pool)
}
