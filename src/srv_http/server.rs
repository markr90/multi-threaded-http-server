use std::io::Write;
use std::net;
use http::StatusCode;

use crate::debug;

use super::request::read_http_request;
use super::response::HttpResponse;
use super::service::Route;
use super::workpool::WorkerPool;

pub struct HttpServer {
    pub listeners: Vec<net::TcpListener>,
    pub routes: Vec<Route<'static>>,
    pub worker_pool: WorkerPool,
}

impl HttpServer {
    pub fn run(&self) {
        for listener in &self.listeners {
            println!("Now listening on {}", listener.local_addr().unwrap());
            for stream in listener.incoming() {
                match stream {
                    Ok(s) => self.handle_connection(s),
                    Err(err) => println!("Stream IO Failure: {}", err)
                }
            }
        }
    }

    fn handle_connection(&self, mut stream: net::TcpStream) -> () {
        let request = match read_http_request(&mut stream) {
            Ok(r) => r,
            Err(err) => {
                println!("{}", err);
                stream.write_all(HttpResponse::new(StatusCode::BAD_REQUEST).build().as_bytes()).unwrap();
                return
            }
        };

        debug!("Received request");
        debug!(&request);

        let found_route = self.routes.iter().find(|&r| r.uri == request.target && r.method == request.method);

        if let Some(route) = found_route {
            let handler_cloned = route.handler.clone();
            self.worker_pool.execute(move || {
                let response = handler_cloned.respond(request);
                stream.write_all(&response.build().as_bytes()).unwrap()
            });
        } else {
            stream.write_all(HttpResponse::new(StatusCode::NOT_FOUND).build().as_bytes()).unwrap();
        }
    }
}
