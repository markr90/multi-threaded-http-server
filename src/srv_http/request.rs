use core::fmt;
use std::io;
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

pub fn read_http_request(stream: &mut net::TcpStream) -> Result<HttpRequest, ParseError> {
    let mut buf_reader = io::BufReader::new(stream);

    let mut request_line = String::new();
    buf_reader.read_line(&mut request_line).map_err(|_| ParseError::RequestLine)?;

    let mut request_line_parts = request_line.split_whitespace();
    let method = request_line_parts.next().ok_or(ParseError::Method)?;
    let path = request_line_parts.next().ok_or(ParseError::Uri)?;
    let version = request_line_parts.next().ok_or(ParseError::Version)?;

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
        target: String::from(path),
        version: parse_http_version(version)?,
        headers: headers,
        body: String::from_utf8(body).map_err(|_| ParseError::Body)?
    };

    Ok(request)
}

// fn read_request_chunks(stream: &mut net::TcpStream, delimiter: u8) -> io::Result<Vec<u8>> {
//     let mut buffer = vec![0u8; 1024];
//     let mut bytes = Vec::new();
//     loop {
//         let n = stream.read(&mut buffer)?;
//         if n == 0 {
//             break; // end of stream
//         }
//         bytes.extend_from_slice(&buffer[..n]);
//         if bytes.ends_with(&[delimiter]) {
//             break; // end of request
//         }
//     }
//     Ok(bytes)
// }

// fn read_until_newline(stream: &mut net::TcpStream) -> Vec<u8> {
//     let mut reader = io::BufReader::new(stream);
//     let mut buf = [0u8; 1024];
//     loop {
//         let num_bytes = reader.read_until(b'\n', &mut buf).unwrap();
//         if num_bytes == 0 {
//             break;
//         }
//         if buf[num_bytes - 1] == b'\n' {
//             buf.truncate(num_bytes);
//             break;
//         }
//     }
//     buf
// }


// optimize later
// pub fn read_http_request(stream: &mut net::TcpStream) -> Result<HttpRequest, ParseError> {
//     let mut buffer = [0; 1024];
//     // Read the stream into the buffer
//     let bytes_read = stream.read(&mut buffer).map_err(|_| ParseError::IO)?;

//     // Parse the request
//     let request_str = String::from_utf8(buffer[..bytes_read].to_vec()).map_err(|_| ParseError::IO)?;

//     println!("{:#?}", request_str);

//     // Extract the request line
//     let mut lines = request_str.lines();
//     let request_line = lines.next().ok_or(ParseError::RequestLine)?;
//     let mut request_parts = request_line.split_whitespace();
//     let method = request_parts.next().ok_or(ParseError::Method)?;
//     let path = request_parts.next().ok_or(ParseError::Uri)?;
//     let version = request_parts.next().ok_or(ParseError::Version)?;

//     // read headers
//     let mut headers: Vec<(String, String)> = Vec::new();
//     let mut content_length: usize = 0;
//     for line in lines {
//         if line.is_empty() {
//             break;
//         }

//         let mut header_parts = line.splitn(2, ": ");
//         let header_name = String::from(header_parts.next().ok_or(ParseError::Headers)?);
//         let header_value = String::from(header_parts.next().ok_or(ParseError::Headers)?);

//         if header_name == "Content-Length" {
//             if let Ok(len) = header_value.parse::<usize>() {
//                 content_length = len;
//             }
//         }

//         headers.push((header_name, header_value));
//     }

//     // read_body

//     let mut body = vec![0; content_length];
//     if content_length > 0 {
//         println!("{:#?}", content_length);
//         stream.read_exact(&mut body).map_err(|_| ParseError::Body)?;
//         println!("here2");
//     }

//     let request = HttpRequest {
//         method: parse_http_method(method)?,
//         target: String::from(path),
//         version: parse_http_version(version)?,
//         body: String::from_utf8(body).map_err(|_| ParseError::Body)?
//     };

//     Ok(request)
// }
