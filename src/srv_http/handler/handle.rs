use std::io;
use std::thread;
use std::time::Duration;
use std::io::prelude::*;
use std::net;
use http::StatusCode;
use super::request::{HttpRequest, read_http_request};
use super::response::HttpResponse;

pub fn handle_connection(mut stream: net::TcpStream) -> io::Result<()> {        
    let response = match read_http_request(&mut stream) {
        Ok(request) => {                
            println!("Request: {}", request);
            process_request(&request)
        },
        Err(err) => {
            println!("Error: {:#?}", err);
            HttpResponse::new(StatusCode::BAD_REQUEST)
        },
    };

    stream.write_all(response.build().as_bytes())
}


fn process_request(request: &HttpRequest) -> HttpResponse {
    match &request.target[..] {
        "/" => HttpResponse::new(StatusCode::OK),
        "/sleep" => {
            thread::sleep(Duration::from_secs(5));
            HttpResponse::new(StatusCode::OK)
        }
        _ => HttpResponse::new(StatusCode::NOT_FOUND)
    }
}