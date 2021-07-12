use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use sqlx::SqlitePool;
use std::str::FromStr;

use crate::buisness;

const CREATE_TABLE: &str = r#"CREATE TABLE IF NOT EXISTS users(id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL, name TEXT NOT NULL)"#;

pub struct SqliteStore(SqlitePool);

impl buisness::Store for SqliteStore {
    fn store(&self, name: String) {
        // todo: SYNC FOR STORE
        println!("store impl name: {}", name);
        sqlx::query!(r#"INSERT INTO users (name) VALUES (?1)"#, name).execute(&self.0);
    }
}

pub async fn new_store() -> Result<Box<dyn buisness::Store + Send + Sync>, sqlx::Error> {
    prepare().await.expect("wrong prepared db");

    let conn = SqliteConnectOptions::from_str("sqlite:db.sqlite3")?.create_if_missing(true);

    let pool = SqlitePoolOptions::new()
        .connect_with(conn)
        .await
        .expect("pool failed");

    let store = SqliteStore(pool);
    Ok(Box::new(store))
}

async fn prepare() -> Result<(), sqlx::Error> {
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

    sqlx::query!(r#"INSERT INTO users (name) VALUES (?1)"#, name)
        .execute(&pool)
        .await
        .expect("not added user to user table");

    Ok(())
}
