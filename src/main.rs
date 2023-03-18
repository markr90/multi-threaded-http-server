use http::StatusCode;
use srv_http::{server_builder::HttpServerBuilder, service::Responder, response::HttpResponse};
use std::thread;
use std::time;
pub mod srv_http;


async fn okFn() -> impl Responder {
    HttpResponse::new(StatusCode::OK)
}

async fn sleepFn() -> impl Responder {
    thread::sleep(time::Duration::from_secs(5));
    HttpResponse::new(StatusCode::OK)
}

fn main() {
    let server = HttpServerBuilder::new()
        .bind("127.0.0.1:3001")
        .add_route("/", okFn)
        .build();
    server.run();
}
