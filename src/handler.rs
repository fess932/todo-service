use rocket::data::{Data, ToByteUnit};
use rocket::tokio;

#[post("/hello", data = "<data>")]
pub async fn hello_world(data: Data<'_>) -> std::io::Result<()> {
    data.open(512.kibibytes())
        .stream_to(tokio::io::stdout())
        .await?;
    Ok(())

    // let mut response = Response::new(Body::empty());

    // match (req.method(), req.uri().path()) {
    //     (&Method::GET, "/") => *response.body_mut() = Body::from("Try posting data to /echo"),
    //     (&Method::POST, "/echo") => {
    //         *response.body_mut() = req.into_body();
    //     }
    //     (&Method::POST, "/echo/uppercase") => {
    //         let mapping = req.into_body().map_ok(|chunk| {
    //             chunk
    //                 .iter()
    //                 .map(|byte| byte.to_ascii_uppercase())
    //                 .collect::<Vec<u8>>()
    //         });

    //         *response.body_mut() = Body::wrap_stream(mapping);
    //     }
    //     (&Method::POST, "/echo/reverse") => {
    //         let full_body = hyper::body::to_bytes(req.into_body()).await?;

    //         let reversed = full_body.iter().rev().cloned().collect::<Vec<u8>>();

    //         *response.body_mut() = reversed.into();
    //     }
    //     _ => {
    //         *response.status_mut() = StatusCode::NOT_FOUND;
    //     }
    // };

    // Ok(response)
}
