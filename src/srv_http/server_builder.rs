use super::{
    server::{HttpServer, RouteAddress, RegexRoute}, service::Route, workpool::WorkerPool,
};
use std::net;

pub struct HttpServerBuilder {
    bindings: Vec<net::SocketAddr>,
    routes: Vec<RegexRoute>,
    worker_pool_limit: usize,
}

// add_middleware

const WORKER_POOL_LIMIT_DEFAULT: usize = 16;

impl HttpServerBuilder {
    pub fn new() -> Self {
        HttpServerBuilder {
            bindings: Vec::new(),
            routes: Vec::new(),
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

    pub fn add_route(mut self, route: Route<'static>) -> Self {
        let regex_route = RegexRoute {
            uri: RouteAddress::new(route.uri),
            method: route.method,
            handler: route.handler,
        };
        self.routes.push(regex_route);
        self
    }

    pub fn build(self) -> HttpServer {
        let mut listeners: Vec<net::TcpListener> = Vec::new();
        for binding in &self.bindings {
            listeners.push(net::TcpListener::bind(binding).unwrap());
        }

        HttpServer {
            listeners,
            routes: self.routes,
            worker_pool: WorkerPool::new(self.worker_pool_limit),
        }
    }
}
