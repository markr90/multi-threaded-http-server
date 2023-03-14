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
        let (mut stream, request) = decode(stream)?;
    
        let header = http_request.first().ok_or(io::Error::new(io::ErrorKind::ConnectionRefused, "Hello"))?;
    
        let mut request_line_items = header.split_whitespace();
        let method = request_line_items.next()?;
        let target = request_line_items.next()?;
        let version = request_line_items.next()?;
    
        let request = HttpRequest { 
            method: parse_http_method(method).map_err(|err| io::Error::new(io::ErrorKind::ConnectionRefused, err))?, 
            target: String::from(target), 
            version: parse_http_version(version).map_err(|err| io::Error::new(io::ErrorKind::ConnectionRefused, err))?, 
        };
    
        let response = HttpResponse::new(StatusCode::OK);
        //  match parse_tcp_stream(stream) {
        //     Ok(request) => {
        //         println!("Request: {}", request);
        //         HttpResponse::new(StatusCode::OK)
        //     },
        //     Err(err) => {
        //         println!("Error: {:#?}", err);
        //         HttpResponse::new(StatusCode::BAD_REQUEST)
        //     },
        // };
        stream.write_all(response.build().as_bytes())
    }
}