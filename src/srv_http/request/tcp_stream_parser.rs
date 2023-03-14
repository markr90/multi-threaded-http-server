use std::{    
    io::{prelude::*, BufReader},
    net::{TcpStream},
};

use super::http_request::{HttpMethod, HttpVersion, HttpRequest};

#[derive(Debug, Clone)]
pub struct TcpStreamError(String);

fn parse_http_method(method: &str) -> Result<HttpMethod, TcpStreamError> {
    match method {
        "GET" => Ok(HttpMethod::GET),
        "POST" => Ok(HttpMethod::POST),
        "PUT" => Ok(HttpMethod::PUT),
        "DELETE" => Ok(HttpMethod::DELETE),
        _ => Err(TcpStreamError(format!("Unknown HTTP Method {}", method)))
    }
}

fn parse_http_version(version: &str) -> Result<HttpVersion, TcpStreamError> {
    match version {
        "HTTP/1.1" => Ok(HttpVersion::OneOne),
        "HTTP/2" => Ok(HttpVersion::Two),
        "HTTP/3" => Ok(HttpVersion::Three),
        _ => Err(TcpStreamError(format!("Unknown HTTP Version {}", version)))
    }
}

pub fn parse_tcp_stream(stream: &TcpStream) -> Result<HttpRequest, TcpStreamError> {
    let buf_reader = BufReader::new(stream);

    let request_line = buf_reader.lines().next()
        .ok_or(TcpStreamError(String::from("stream is empty")))?
        .map_err(|err| TcpStreamError(format!("{}", err)))?;

    let mut request_line_items = request_line.split_whitespace();
    let method = request_line_items.next().ok_or(TcpStreamError(String::from("no method found")))?;
    let target = request_line_items.next().ok_or(TcpStreamError(String::from("no target found")))?;
    let version = request_line_items.next().ok_or(TcpStreamError(String::from("no version found")))?;

    let request = HttpRequest { 
        method: parse_http_method(method)?, 
        target: String::from(target), 
        version: parse_http_version(version)?, 
    };
    Ok(request)
}