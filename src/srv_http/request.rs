use core::fmt;
use std::{io, collections::HashMap};
use std::io::prelude::*;
use std::net;

use super::http_constants::{HttpMethod, HttpVersion};

#[derive(Debug)]
pub enum ParseError {
    RequestLine,
    Method,
    Version,
    Uri,
    Headers,
    Body,
}

impl std::error::Error for ParseError {}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match &self {
            ParseError::RequestLine =>"Invalid Request Line",
            ParseError::Method =>"Invalid Method",
            ParseError::Version => "Invalid Version",
            ParseError::Uri => "Invalid Uri",
            ParseError::Headers => "Invalid Headers",
            ParseError::Body => "Invalid Body",
        })
    }
}

#[derive(Debug)]
pub struct HttpRequest {
    pub method: HttpMethod,
    pub target: String,
    pub version: HttpVersion,
    pub headers: Vec<(String, String)>,
    pub body: String,
    pub query_params: HashMap<String, String>,
}

impl fmt::Display for HttpRequest {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {}\r\n", &self.method.to_string(), &self.target.to_string(), &self.version.to_string())?;
        for header in &self.headers {
            write!(f, "{}: {}\r\n", header.0, header.1)?;
        }
        write!(f, "{}", &self.body)
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

const INVALID_URI_CHARS: [&str; 11] = ["\"", "<", ">", "[", "]", "\\", "^", "`", "{", "|", "}"];

fn is_valid_uri(uri: &str) -> bool {
    let mut is_valid = true;
    for c in INVALID_URI_CHARS {
        is_valid &= !uri.contains(c);
    }

    is_valid
}

pub fn read_http_request(stream: &mut net::TcpStream) -> Result<HttpRequest, ParseError> {
    let mut buf_reader = io::BufReader::new(stream);

    let mut request_line = String::new();
    buf_reader.read_line(&mut request_line).map_err(|_| ParseError::RequestLine)?;

    let mut request_line_parts = request_line.split_whitespace();
    let method = request_line_parts.next().ok_or(ParseError::Method)?;
    let path = request_line_parts.next().ok_or(ParseError::Uri)?;

    if !is_valid_uri(path) {
        return Err(ParseError::Uri);
    }

    let version = request_line_parts.next().ok_or(ParseError::Version)?;

    let mut uri_parts = path.split("?");
    let target = uri_parts.next().ok_or(ParseError::Uri)?;
    let query_param_part = uri_parts.next();

    let mut query_params: HashMap<String, String> = HashMap::new();
    if let Some(qp) = query_param_part {
        for q in qp.split("&") {
            let mut qsplit = q.split("=");
            let qp_name = qsplit.next().ok_or(ParseError::Uri)?;
            let qp_value = match qsplit.next() {
                Some(v) => v,
                None => "",
            };
            query_params.insert(qp_name.to_string(), qp_value.to_string());
        }
    }

    // read headers
    let mut headers: Vec<(String, String)> = Vec::new();
    let mut content_length: usize = 0;
    let mut line: String;

    loop {
        line = String::new();
        buf_reader.read_line(&mut line).map_err(|_| ParseError::Headers)?;
        if line == "\r\n" {
            break;
        }

        let mut header_parts = line.trim_end().splitn(2, ": ");
        let header_name = String::from(header_parts.next().ok_or(ParseError::Headers)?);
        let header_value = String::from(header_parts.next().ok_or(ParseError::Headers)?);

        if header_name == "Content-Length" {
            if let Ok(len) = header_value.parse::<usize>() {
                content_length = len;
            }
        }

        headers.push((header_name, header_value));
    }

    // read_body
    let mut body = vec![0; content_length];
    if content_length > 0 {
        buf_reader.read_exact(&mut body).map_err(|_| ParseError::Body)?;
    }

    let request = HttpRequest {
        method: parse_http_method(method)?,
        target: String::from(target),
        version: parse_http_version(version)?,
        headers,
        body: String::from_utf8(body).map_err(|_| ParseError::Body)?,
        query_params,
    };

    Ok(request)
}
