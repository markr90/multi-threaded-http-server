use http::StatusCode;

use super::{
    request::HttpRequest, 
    response::HttpResponse
};

pub type Service = Box<dyn Responder + Send + 'static>;

pub trait Responder {
    fn respond(&self, req: &HttpRequest) -> HttpResponse;
}

impl Responder for HttpResponse {
    fn respond(&self, _: &HttpRequest) -> HttpResponse {
        HttpResponse::new(StatusCode::OK)
    }
}