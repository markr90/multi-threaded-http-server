use std::io::Write;
use std::net;
use std::sync::Arc;
use http::StatusCode;
use regex::Regex;

use crate::debug;

use super::http_constants::HttpMethod;
use super::request::read_http_request;
use super::response::HttpResponse;
use super::service::{Route, RouteHandler};
use super::workpool::WorkerPool;

pub struct HttpServer {
    pub listeners: Vec<net::TcpListener>,
    pub routes: Vec<RegexRoute>,
    pub worker_pool: WorkerPool,
}

pub struct RouteAddress {
    pub uri_template: String,
    pub uri_regex: regex::Regex,
}

impl RouteAddress {
    pub fn new(uri_template: String) -> Self {
        let mut url_params: Vec<String> = Vec::new();

        let mut url_param = String::new();
        let mut is_url_param = false;
        let mut regex_template = String::new();
        for c in uri_template.chars() {
            if c == '{' {
                is_url_param = true;
            } else if c == '}' {
                url_params.push(url_param);
                url_param = String::new();
                is_url_param = false;
                regex_template += "(.*)";
            } else if is_url_param {
                url_param.push(c);
            } else {
                regex_template.push(c);
            }
        }
        regex_template.push('$');

        let regex_escaped: String = regex::escape(&uri_template);
        let uri_regex = Regex::new(format!(r"{}", &regex_escaped).as_str()).unwrap();
        println!("{}", uri_regex);

        RouteAddress {
            uri_template,
            uri_regex,
        }
    }

    pub fn is_match(&self, uri: &String) -> bool {
        println!("{}", uri);
        println!("{}", self.uri_regex);
        println!("{}", self.uri_regex.is_match(uri));
        self.uri_regex.is_match(uri)
    }
}

pub struct RegexRoute {
    pub uri: RouteAddress,
    pub method: HttpMethod,
    pub handler: Arc<Box<dyn RouteHandler>>,
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

        debug!(&request);

        let found_route = self.routes.iter().find(|&r| r.uri.is_match(&request.target) && r.method == request.method);

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
