use http::StatusCode;
use serde::Serialize;
use crate::debug;

pub struct HttpResponse {
    pub status_code: StatusCode,
    pub body: String,
    pub headers: Vec<(String, String)>,
}

impl HttpResponse {
    pub fn new(status_code: StatusCode) -> Self {
        HttpResponse {
            status_code,
            body: String::new(),
            headers: Vec::new(),
        }
    }

    pub fn build(self) -> String {
        let response = concat_string!(
            "HTTP/1.1 ",
            self.status_code.to_string(),
            "\r\n",
            self.build_headers(),
            "\r\n",
            self.body
        );
        debug!(&response);
        response
    }

    pub fn json<T: Serialize>(mut self, obj: &T) -> Self {
        self.add_header(String::from("Content-Type"), String::from("application/json; charset=utf-8"));
        match serde_json::to_string(obj) {
            Ok(o) => self.body = o,
            Err(_) => self.status_code = StatusCode::INTERNAL_SERVER_ERROR,
        }
        if !self.body.is_empty() {
            self.add_header(String::from("Content-Length"), self.body.len().to_string());
        }
        self
    }

    fn add_header(&mut self, name: String, value: String) -> () {
        self.headers.push((name, value));
    }

    fn build_headers(&self) -> String {
        self.headers
            .iter()
            .map(|h| concat_string!(h.0, ": ", h.1, "\r\n"))
            .collect()
    }
}
