use std::thread;
use std::time::Duration;
use std::io::prelude::*;
use std::net;
use http::StatusCode;
use super::request::{HttpRequest, read_http_request};
use super::response::HttpResponse;
use super::workpool::WorkerPool;

// bind
// listen
// add_middleware
// add_service
// run

pub struct HttpServer {
    listeners: Vec<net::TcpListener>,
    worker_pool: WorkerPool,
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
            listeners: Vec::new(),
            worker_pool: WorkerPool::new(8),
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
                    Ok(s) => self.handle_connection(s),
                    Err(err) => println!("Stream IO Failure: {}", err)
                }
            }
        }
    }

    fn handle_connection<'a>(&'a self, mut stream: net::TcpStream) -> () {
        self.worker_pool.execute(move || { 
            let request = read_http_request(&mut stream);
            let response = match request {
                Ok(r) => process_request(&r),
                Err(_) => HttpResponse::new(StatusCode::BAD_REQUEST),
            };
            
            stream.write_all(response.build().as_bytes()).unwrap();
        });
    }
}