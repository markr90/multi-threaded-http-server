use std::net::{TcpListener};

use super::request::tcp_stream_parser::parse_tcp_stream;

// bind
// listen
// add_middleware
// add_service
// run

pub struct HttpServer {
    app: String,
    listeners: Vec<TcpListener>
}

impl HttpServer {
    pub fn new(app_name: &str) -> Self {
        println!("Starting {}", app_name);
        HttpServer {
            app: String::from(app_name),
            listeners: Vec::new()
        }
    }

    // todo add error handling
    pub fn bind(mut self, address: &str) -> Self {
        let listener = TcpListener::bind(address).unwrap();
        self.listeners.push(listener);
        self
    }

    pub fn run(self) {
        for listener in self.listeners {
            for stream in listener.incoming() {
                let stream = stream.unwrap();
    
                match parse_tcp_stream(&stream) {
                    Ok(request) => println!("Request received: {}", request),
                    Err(err) => println!("Error: {:#?}", err),
                }
    
                // handle_connection(stream);
            }
        }
    }
}