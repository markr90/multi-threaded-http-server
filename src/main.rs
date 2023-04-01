use srv_http::request::HttpMethod;
use srv_http::service::{HomeHandler, SleepHandler, Route};
use srv_http::server_builder::HttpServerBuilder;
pub mod srv_http;

fn main() {
    let server = HttpServerBuilder::new()
        .bind("127.0.0.1:3001")
        .add_route(Route::new("/", HttpMethod::GET, HomeHandler))
        .add_route(Route::new("/sleep", HttpMethod::GET, SleepHandler))
        .build();
    server.run();
}
