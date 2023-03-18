use std::io;
use std::thread;
use std::time::Duration;
use std::io::prelude::*;
use std::net;
use http::StatusCode;

use super::{
    handler::handle::handle_connection
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