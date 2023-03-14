use srv_http::server::{HttpServer};
pub mod srv_http;

fn main() {
    HttpServer::new()
        .bind("127.0.0.1:3001")
        .run();
}
