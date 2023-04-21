use core::fmt;
use std::io;
use std::io::prelude::*;

use crate::debug;

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
    pub uri: String,
    pub version: HttpVersion,
    pub headers: Vec<(String, String)>,
    pub body: String,
    pub query_params: Vec<(String, String)>,
}

impl fmt::Display for HttpRequest {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {}\r\n", &self.method.to_string(), &self.uri.to_string(), &self.version.to_string())?;
        for header in &self.headers {
            write!(f, "{}: {}\r\n", header.0, header.1)?;
        }
        write!(f, "{}", &self.body)
    }
}


fn parse_http_method(method: &str) -> Result<HttpMethod, ParseError> {
    debug!(method);
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

pub fn read_http_request<T>(stream: &mut T) -> Result<HttpRequest, ParseError>
where T: std::io::Read {
    let mut buf_reader = io::BufReader::new(stream);

    let mut request_line = String::new();
    buf_reader.read_line(&mut request_line).map_err(|_| ParseError::RequestLine)?;

    debug!(&request_line);
    let mut request_line_parts = request_line.split_whitespace();
    let method = request_line_parts.next().ok_or(ParseError::Method)?;
    let path = request_line_parts.next().ok_or(ParseError::Uri)?;

    if !is_valid_uri(path) {
        return Err(ParseError::Uri);
    }

    let version = request_line_parts.next().ok_or(ParseError::Version)?;

    // parse uri and query parameters
    let mut uri_parts = path.split("?");
    let target = uri_parts.next().ok_or(ParseError::Uri)?;
    let query_param_part = uri_parts.next();

    let mut query_params: Vec<(String, String)> =  Vec::new();
    if let Some(qp) = query_param_part {
        for q in qp.split("&") {
            let mut qsplit = q.split("=");
            let qp_name = qsplit.next().ok_or(ParseError::Uri)?;
            let qp_value = match qsplit.next() {
                Some(v) => v,
                None => "",
            };
            query_params.push((qp_name.to_string(), qp_value.to_string()));
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
        uri: String::from(target),
        version: parse_http_version(version)?,
        headers,
        body: String::from_utf8(body).map_err(|_| ParseError::Body)?,
        query_params,
    };

    Ok(request)
}

#[allow(dead_code)]
const TEST_REQUEST: &str = "GET /request?qp1=1&qp2=2 HTTP/1.1\r\n\
header1: header1\r\n\
header2: header2\r\n\
Content-Length: 22\r\n\
\r\n\
<body>some_body</body>\r\n\
\r\n\r\n
";

#[test]
fn parses_method() {
    let http_request = read_http_request(&mut TEST_REQUEST.as_bytes()).unwrap();
    assert_eq!(http_request.method, HttpMethod::GET);
}

#[test]
fn parses_uri() {
    let http_request = read_http_request(&mut TEST_REQUEST.as_bytes()).unwrap();
    assert_eq!(http_request.uri, "/request");
}

#[test]
fn parses_query_params() {
    let http_request = read_http_request(&mut TEST_REQUEST.as_bytes()).unwrap();
    let mut query_params = http_request.query_params.iter();
    let first_qp = query_params.next().unwrap();
    let second_qp = query_params.next().unwrap();
    assert_eq!(first_qp, &("qp1".to_string(), "1".to_string()));
    assert_eq!(second_qp, &("qp2".to_string(), "2".to_string()));
}

#[test]
fn parses_version() {
    let http_request = read_http_request(&mut TEST_REQUEST.as_bytes()).unwrap();
    assert_eq!(http_request.version, HttpVersion::Http11);
}

#[test]
fn parses_headers() {
    let http_request = read_http_request(&mut TEST_REQUEST.as_bytes()).unwrap();
    let mut headers = http_request.headers.iter();
    let first_header = headers.next().unwrap();
    let second_header = headers.next().unwrap();
    assert_eq!(first_header, &("header1".to_string(), "header1".to_string()));
    assert_eq!(second_header, &("header2".to_string(), "header2".to_string()));
}

#[test]
fn parses_body() {
    let http_request = read_http_request(&mut TEST_REQUEST.as_bytes()).unwrap();
    let expected = "<body>some_body</body>";
    assert_eq!(http_request.body, expected);
}

