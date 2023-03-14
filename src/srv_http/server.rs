use std::io;
use std::io::prelude::*;
use std::net;
use http::StatusCode;

use super::{
    request::http_request::decode,
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

    pub fn run(self) {
        for listener in &self.listeners {
            for stream in listener.incoming() {
                let stream = stream.unwrap();
                self.handle_connection(stream).unwrap();
            }
        }
    }


    fn handle_connection(&self, mut stream: net::TcpStream) -> io::Result<()> {        
        let response = match decode(&mut stream) {
            Ok(request) => {
                println!("Request: {}", request);
                HttpResponse::new(StatusCode::OK)
            },
            Err(err) => {
                println!("Error: {:#?}", err);
                HttpResponse::new(StatusCode::BAD_REQUEST)
            },
        };

        stream.write_all(response.build().as_bytes())
    }
}