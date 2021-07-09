use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use std::convert::Infallible;
use std::net::SocketAddr;
use sqlx::sqlite::SqlitePoolOptions;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let pool = SqlitePoolOptions::new().connect("sqlite:db.sqlite3").await?;

    // let row: (i64,) = sqlx::query_as("SELECT $1")
    //     .bind(150_i64)
    //     .fetch_one(&pool).await?;

    // assert_eq!(row.0, 150);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3001));

    let make_svc = make_service_fn(|_conn| async { Ok::<_, Infallible>(service_fn(hello_world)) });

    let server = Server::bind(&addr).serve(make_svc);

    let graceful = server.with_graceful_shutdown(shutdown_signal());

    if let Err(e) = graceful.await {
        eprintln!("server error: {}", e);
    }

    Ok(())
}

async fn hello_world(_req: Request<Body>) -> Result<Response<Body>, Infallible> {
    Ok(Response::new("Hello, World".into()))
}

async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("failed to install CTRL+C signal hanlder");
    println!("gracefull shutdown by CTRL+C");
}
