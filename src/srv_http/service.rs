use std::{thread, time, sync::Arc};

use http::StatusCode;

use super::{
    request::{HttpRequest, HttpMethod},
    response::HttpResponse
};

pub trait RouteHandler: Sync + Send {
    fn respond(&self, request: HttpRequest) -> HttpResponse;
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

pub struct HomeHandler;
impl RouteHandler for HomeHandler {
    fn respond(&self, _: HttpRequest) -> HttpResponse {
        HttpResponse::new(StatusCode::OK)
    }
}

pub struct SleepHandler;
impl RouteHandler for SleepHandler {
    fn respond(&self, _: HttpRequest) -> HttpResponse {
        thread::sleep(time::Duration::from_secs(5));
        HttpResponse::new(StatusCode::OK)
    }
}
