use hyper::service::{make_service_fn, service_fn};
use hyper::Server;
use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use std::convert::Infallible;
use std::net::SocketAddr;
use std::str::FromStr;

mod handler;

const CREATE_TABLE: &str =
    "CREATE TABLE IF NOT EXISTS users(id INTEGER PRIMARY KEY NOT NULL, name TEXT NOT NULL)";

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    prepare().await.expect("wrong prepared db");
    // sqlx::query(
    //     "CREATE TABLE IF NOT EXISTS users(id INTEGER PRIMARY KEY NOT NULL, name TEXT NOT NULL)",
    // )
    // .fetch(&pool)
    // .await?;

    // let row: (i64,) = sqlx::query_as("SELECT $1")
    //     .bind(150_i64)
    //     .fetch_one(&pool)
    //     .await?;

    // assert_eq!(row.0, 150);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3001));

    let make_svc =
        make_service_fn(|_conn| async { Ok::<_, Infallible>(service_fn(handler::hello_world)) });

    let server = Server::bind(&addr).serve(make_svc);

    let graceful = server.with_graceful_shutdown(shutdown_signal());

    if let Err(e) = graceful.await {
        eprintln!("server error: {}", e);
    }

    Ok(())
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

async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("failed to install CTRL+C signal hanlder");
    println!("gracefull shutdown by CTRL+C");
}
