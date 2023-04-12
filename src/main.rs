use services::example_services::{HomeHandler, SleepHandler, AnimalResponder};
use srv_http::http_constants::HttpMethod;
use srv_http::server_builder::HttpServerBuilder;
use srv_http::service::Route;
pub mod srv_http;
pub mod services;
pub mod util;
#[macro_use(concat_string)]
extern crate concat_string;

fn main() {
    let server = HttpServerBuilder::new()
        .bind("127.0.0.1:3001")
        .worker_pool_limit(32)
        .add_route(Route::new("/", HttpMethod::GET, HomeHandler))
        .add_route(Route::new("/sleep", HttpMethod::GET, SleepHandler))
        .add_route(Route::new("/animal", HttpMethod::POST, AnimalResponder))
        .build();
    server.run();
}
