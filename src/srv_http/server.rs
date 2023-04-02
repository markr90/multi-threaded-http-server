use std::collections::HashMap;
use std::io::Write;
use std::net;
use std::sync::Arc;
use http::StatusCode;
use regex::Regex;

use crate::debug;

use super::http_constants::HttpMethod;
use super::request::{read_http_request, ParseError};
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
    pub uri_params: Vec<String>,
}

impl RouteAddress {
    pub fn new(uri_template: String) -> Self {
        let mut uri_params: Vec<String> = Vec::new();

        let mut url_param = String::new();
        let mut is_url_param = false;
        let mut regex_template = String::new();
        for c in uri_template.chars() {
            if c == '{' {
                is_url_param = true;
            } else if c == '}' {
                uri_params.push(url_param);
                url_param = String::new();
                is_url_param = false;
                regex_template += "([^/]+)";
            } else if is_url_param {
                url_param.push(c);
            } else {
                regex_template.push(c);
            }
        }
        regex_template.push('$');

        let uri_regex = Regex::new(format!(r"{}", &regex_template).as_str()).unwrap();

        RouteAddress {
            uri_template,
            uri_regex,
            uri_params,
        }
    }

    pub fn is_match(&self, uri: &String) -> bool {
        self.uri_regex.is_match(uri)
    }

    pub fn extract_uri_params(&self, uri: &String) -> Result<HashMap<String, String>, ParseError> {
        let cap_groups = match self.uri_regex.captures(uri) {
            Some(cg) => cg,
            None => return Err(ParseError::Uri),
        };
        let mut uri_params_extracted: HashMap<String, String> = HashMap::new();
        for (i, uri_param) in self.uri_params.iter().enumerate() {
            let uri_param_value = match cap_groups.get(i + 1) {
                Some(v) => v,
                None => return Err(ParseError::Uri),
            };
            uri_params_extracted.insert(uri_param.clone(), uri_param_value.as_str().to_string());
        }

        Ok(uri_params_extracted)
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

        let mut found_routes = self.routes.iter().filter(|r| r.uri.is_match(&request.target)).peekable();
        if found_routes.peek().is_none() {
            stream.write_all(HttpResponse::new(StatusCode::NOT_FOUND).build().as_bytes()).unwrap();
            return;
        }

        let found_route = found_routes.find(|r| r.method == request.method);

        if let Some(route) = found_route {
            debug!(route.uri.extract_uri_params(&request.target));
            // debug!(route.uri.extract_uri_params(&request.target).unwrap());
            let handler_cloned = route.handler.clone();
            self.worker_pool.execute(move || {
                let response = handler_cloned.respond(request);
                stream.write_all(&response.build().as_bytes()).unwrap()
            });
        } else {
            stream.write_all(HttpResponse::new(StatusCode::METHOD_NOT_ALLOWED).build().as_bytes()).unwrap();
        }
    }
}
