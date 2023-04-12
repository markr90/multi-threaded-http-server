use std::{thread, time};
use http::StatusCode;
use serde::{Deserialize, Serialize};
use crate::srv_http::{service::{RouteHandler, ServerError}, response::HttpResponse, request::HttpRequest};

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

#[derive(Deserialize)]
pub struct AnimalRequest {
    id: u64,
}

#[derive(Serialize)]
pub struct AnimalResponse {
    name: String,
}

pub struct AnimalResponder;

impl RouteHandler for AnimalResponder {
    fn respond(&self, request: HttpRequest) -> HttpResponse {
        let request = serde_json::from_str(&request.body);
        let request: AnimalRequest = match request {
            Ok(r) => r,
            Err(_) => return HttpResponse::new(StatusCode::BAD_REQUEST),
        };

        let response = match request.id {
            1 => Ok(AnimalResponse { name: String::from("Dog") }),
            2 => Ok(AnimalResponse { name: String::from("Cat") }),
            _ => Err(ServerError::Fail),
        };

        match response {
            Ok(r) => HttpResponse::new(StatusCode::OK).json(&r),
            Err(_) => HttpResponse::new(StatusCode::NOT_FOUND),
        }
    }
}
