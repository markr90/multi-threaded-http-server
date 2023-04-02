use std::sync::Arc;
use serde::de::DeserializeOwned;

use super::{
    request::HttpRequest,
    response::HttpResponse, http_constants::HttpMethod
};

pub trait RouteHandler: Sync + Send {
    fn respond(&self, request: HttpRequest) -> HttpResponse;
}

#[derive(Debug)]
pub enum ServerError {
    Fail,
}

// pub trait JsonRouteHandler<T: Deserialize<'static>, V: Serialize>: RouteHandler {
pub trait JsonRouteHandler: RouteHandler {
    type RequestObject: DeserializeOwned;
    fn process(&self, body: Self::RequestObject) -> HttpResponse;
}

impl<'a, R> RouteHandler for R
where
    R: JsonRouteHandler
{
    fn respond(&self, request: HttpRequest) -> HttpResponse {
        // Implement the `respond` method for `RouteHandler` here
        // add error handling
        let obj: R::RequestObject = serde_json::from_str(&request.body).unwrap();
        self.process(obj)
    }
}


pub struct Route<'a> {
    pub uri: String,
    pub method: HttpMethod,
    pub handler: Arc<Box<dyn RouteHandler + 'a>>,
}

impl<'a> Route<'a> {
    pub fn new<T>(uri: &str, method: HttpMethod, handler: T) -> Self
    where T: RouteHandler + 'a {
        Route {
            uri: String::from(uri),
            method,
            handler: Arc::new(Box::new(handler)),
        }
    }
}

impl<'a> PartialEq for Route<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.uri == other.uri && self.method == other.method
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

pub struct HttpService<'a> {
    routes: Vec<Route<'a>>,
}

impl<'a> HttpService<'a> {
    pub fn new() -> Self {
        HttpService {
            routes: Vec::new()
        }
    }

    pub fn add_route<T>(&mut self, route: Route<'a>) -> () {
        self.routes.push(route);
    }
}
