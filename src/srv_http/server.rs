use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::io::prelude::*;
use std::net;
use http::StatusCode;
use std::{thread, time};
use super::request::{read_http_request};
use super::response::HttpResponse;
use super::service::{Service, Responder};
use super::workpool::WorkerPool;

pub struct HttpServer {
    pub listeners: Vec<net::TcpListener>,
    // pub services: Arc<Mutex<HashMap<String, Service>>>,
    pub services: Arc<Mutex<HashMap<String, Box<dyn FnOnce() + Send + 'static>>>>,
    pub worker_pool: WorkerPool,
}

impl HttpServer {
    pub fn run(&self) {
        for listener in &self.listeners {
            for stream in listener.incoming() {
                match stream {
                    Ok(s) => self.handle_connection(s),
                    Err(err) => println!("Stream IO Failure: {}", err)
                }
            }
        }
    }

    fn handle_connection(&self, mut stream: net::TcpStream) -> () {
        let services = Arc::clone(&self.services);
        self.worker_pool.execute(move || { 
            let request = read_http_request(&mut stream);
            let response = match request {
                Ok(r) => {                    
                    let services = services.lock().unwrap();
                    let service = services.get(&r.target).unwrap();
                    service()
                },
                Err(_) => (), // HttpResponse::new(StatusCode::BAD_REQUEST),
            };
    
            // stream.write_all(&response.build().as_bytes()).unwrap()
        });
    }
}