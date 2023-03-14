use core::fmt;
use std::io;
use std::io::prelude::*;
use std::net;

use crate::srv_http::request::tcp_stream_parser::{
    parse_http_method,
    parse_http_version
};

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
    Http11,
    Http2,
    Http3,
}

impl fmt::Display for HttpVersion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {        
        write!(f, "{}", match &self {
            HttpVersion::Http11 => "HTTP/1.1",
            HttpVersion::Http2 => "HTTP/2",
            HttpVersion::Http3 => "HTTP/3",
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

pub fn decode(mut stream: net::TcpStream) -> io::Result<(net::TcpStream, HttpRequest)> {
    let buf_reader = io::BufReader::new(&mut stream);  
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    println!("Request: {:#?}", http_request);
    
    let header = http_request.first().ok_or(io::Error::new(io::ErrorKind::ConnectionRefused, "Hello"))?;

    let mut request_line_items = header.split_whitespace();
    let method = request_line_items.next()?;
    let target = request_line_items.next()?;
    let version = request_line_items.next()?;

    let request = HttpRequest { 
        method: parse_http_method(method).map_err(|err| io::Error::new(io::ErrorKind::ConnectionRefused, err))?, 
        target: String::from(target), 
        version: parse_http_version(version).map_err(|err| io::Error::new(io::ErrorKind::ConnectionRefused, err))?, 
    };

    Ok((stream, request))
}