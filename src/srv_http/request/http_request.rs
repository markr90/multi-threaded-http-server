use core::fmt;
use std::io;
use std::io::prelude::*;
use std::net;

#[derive(Debug)]
pub enum ParseError {
    Method,
    Version,
    Uri,
    IO,
}

impl std::error::Error for ParseError {}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match &self {
          ParseError::Method =>"Invalid Method",
          ParseError::Version => "Invalid Version",
          ParseError::Uri => "Invalid Uri",
          ParseError::IO => "IO Error",
        })
    }
}

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
    Http10,
    Http11,
}

impl fmt::Display for HttpVersion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {        
        write!(f, "{}", match &self {
            HttpVersion::Http10 => "HTTP/1.0",
            HttpVersion::Http11 => "HTTP/1.1",
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


fn parse_http_method(method: &str) -> Result<HttpMethod, ParseError> {
    match method {
        "GET" => Ok(HttpMethod::GET),
        "POST" => Ok(HttpMethod::POST),
        "PUT" => Ok(HttpMethod::PUT),
        "DELETE" => Ok(HttpMethod::DELETE),
        _ => Err(ParseError::Method)
    }
}

fn parse_http_version(version: &str) -> Result<HttpVersion, ParseError> {
    match version {
        "HTTP/1.0" => Ok(HttpVersion::Http10),
        "HTTP/1.1" => Ok(HttpVersion::Http11),
        _ => Err(ParseError::Version)
    }
}

pub fn decode(stream: &mut net::TcpStream) -> Result<HttpRequest, ParseError> {
    let buf_reader = io::BufReader::new(stream); 
    // todo error handling here 
    let http_request: Vec<String> = buf_reader
        .lines()
        .take_while(|line| match line {
            Ok(line) => !line.is_empty(),
            Err(_) => false
        })
        .collect::<Result<Vec<String>, _>>()
        .map_err(|_| ParseError::IO)?;

    println!("Request: {:#?}", http_request);
    
    let header = http_request.first().unwrap();

    let mut request_line_items = header.split_whitespace();
    let method = request_line_items.next().ok_or(ParseError::Method)?;
    let target = request_line_items.next().ok_or(ParseError::Uri)?;
    let version = request_line_items.next().ok_or(ParseError::Version)?;

    let request = HttpRequest { 
        method: parse_http_method(method)?, 
        target: String::from(target), 
        version: parse_http_version(version)?, 
    };

    Ok(request)
}