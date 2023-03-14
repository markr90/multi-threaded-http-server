use core::fmt;

#[derive(Debug)]
pub enum HttpMethod {
    GET,
    POST,
    PUT,
    DELETE,
}

impl fmt::Display for HttpMethod {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match &self {
            HttpMethod::GET => "GET",
            HttpMethod::POST => "POST",
            HttpMethod::PUT => "PUT",
            HttpMethod::DELETE => "DELETE",
        })
    }
}

#[derive(Debug)]
pub enum HttpVersion {
    OneOne,
    Two,
    Three,
}

impl fmt::Display for HttpVersion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {        
        write!(f, "{}", match &self {
            HttpVersion::OneOne => "HTTP/1.1",
            HttpVersion::Two => "HTTP/2",
            HttpVersion::Three => "HTTP/3",
        })
    }
}

pub struct HttpRequest {
    pub method: HttpMethod,
    pub target: String,
    pub version: HttpVersion,
}

impl fmt::Display for HttpRequest {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {}", &self.method.to_string(), &self.target.to_string(), &self.version.to_string())
    }
}