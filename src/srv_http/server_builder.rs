use http::StatusCode;

use super::{
    server::HttpServer,
    service::{Service, Responder}, workpool::WorkerPool, response::HttpResponse
};
use std::{net, collections::HashMap, sync::{Arc, Mutex}};

pub struct HttpServerBuilder {
    bindings: Vec<net::SocketAddr>,
    //services: HashMap<String, Service>,
    services: HashMap<String, Box<dyn FnOnce() + Send + 'static>>,
    worker_pool_limit: usize,
}

// add_middleware

const WORKER_POOL_LIMIT_DEFAULT: usize = 16;

async fn test() -> impl Responder {
    HttpResponse::new(StatusCode::OK)
}

impl HttpServerBuilder {
    pub fn new() -> Self {
        HttpServerBuilder {
            bindings: Vec::new(),
            services: HashMap::new(),
            worker_pool_limit: WORKER_POOL_LIMIT_DEFAULT,
        }
    }

    pub fn worker_pool_limit(mut self, size: usize) -> Self {
        assert!(size > 0);
        self.worker_pool_limit = size;
        self
    }

    // todo add error handling
    pub fn bind<A: net::ToSocketAddrs>(mut self, address: A) -> Self {
        let mut binding = address
            .to_socket_addrs()
            .expect("Unable to resolve domain");
        match binding.next() {
            Some(b) => self.bindings.push(b),
            None => (),
        }
        self
    }

    // pub fn add_service(mut self, route: &str, service: Service) -> Self {
    //     self.services.insert(String::from(route), service);
    //     self
    // }

    pub fn add_route<F>(mut self, route: &str, service: F) -> Self
    where F: FnOnce() + Send + 'static, {
        self.services.insert(String::from(route), Box::new(service));
        self
    }

    pub fn build(self) -> HttpServer {
        let mut listeners: Vec<net::TcpListener> = Vec::new();
        for binding in &self.bindings {
            listeners.push(net::TcpListener::bind(binding).unwrap());
        }

        HttpServer {
            listeners,
            services: Arc::new(Mutex::new(self.services)),
            worker_pool: WorkerPool::new(self.worker_pool_limit),
        }
    }
}