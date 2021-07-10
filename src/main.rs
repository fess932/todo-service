use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use std::str::FromStr;

mod handler;

const CREATE_TABLE: &str = r#"CREATE TABLE IF NOT EXISTS users(id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL, name TEXT NOT NULL)"#;

#[macro_use]
extern crate rocket;

#[launch]
async fn rocket() -> _ {
    prepare().await.expect("wrong prepared db");

    rocket::build().mount("/", routes![handler::hello_world])
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
