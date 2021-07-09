use rocket::tokio::time::{sleep, Duration};
use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use std::str::FromStr;

mod handler;

const CREATE_TABLE: &str =
    "CREATE TABLE IF NOT EXISTS users(id INTEGER PRIMARY KEY NOT NULL, name TEXT NOT NULL)";

#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello world"
}

#[get("/delay/<seconds>")]
async fn delay(seconds: u64) -> String {
    sleep(Duration::from_secs(seconds)).await;
    format!("Wanted for {} seconds", seconds)
}

#[launch]
async fn rocket() -> _ {
    prepare().await.expect("wrong prepared db");

    rocket::build().mount("/", routes![index, delay, handler::hello_world])
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

    Ok(())
}
