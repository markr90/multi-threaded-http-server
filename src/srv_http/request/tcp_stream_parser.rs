use super::http_request::{HttpMethod, HttpVersion};

pub fn parse_http_method(method: &str) -> Result<HttpMethod, String> {
    match method {
        "GET" => Ok(HttpMethod::GET),
        "POST" => Ok(HttpMethod::POST),
        "PUT" => Ok(HttpMethod::PUT),
        "DELETE" => Ok(HttpMethod::DELETE),
        _ => Err(format!("Unsupported HTTP Method {}", method))
    }
}

pub fn parse_http_version(version: &str) -> Result<HttpVersion, String> {
    match version {
        "HTTP/1.1" => Ok(HttpVersion::Http11),
        "HTTP/2" => Ok(HttpVersion::Http2),
        "HTTP/3" => Ok(HttpVersion::Http3),
        _ => Err(format!("Unsupported HTTP Version {}", version))
    }
}