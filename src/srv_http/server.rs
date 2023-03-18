use std::io;
use std::thread;
use std::time::Duration;
use std::io::prelude::*;
use std::net;
use http::StatusCode;

use super::request::http_request::HttpRequest;
use super::{
    request::http_request::read_http_request,
    response::response::HttpResponse
};

// bind
// listen
// add_middleware
// add_service
// run

pub struct HttpServer {
    listeners: Vec<net::TcpListener>
}

fn handle_connection(mut stream: net::TcpStream) -> io::Result<()> {        
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

impl HttpServer {
    pub fn new() -> Self {
        HttpServer {
            listeners: Vec::new()
        }
    }

    // todo add error handling
    pub fn bind<A: net::ToSocketAddrs>(mut self, address: A) -> Self {
        let listener = net::TcpListener::bind(address).unwrap();
        self.listeners.push(listener);
        self
    }

    pub fn run<'a>(&'a self) {
        for listener in &self.listeners {
            for stream in listener.incoming() {
                match stream {
                    Ok(s) => self.send_stream_to_worker(s),
                    Err(err) => println!("Stream IO Failure: {}", err)
                }
            }
        }
    }

    fn send_stream_to_worker<'a>(&'a self, stream: net::TcpStream) -> () {
        thread::spawn(|| { 
            handle_connection(stream).unwrap(); 
        });
    }
}