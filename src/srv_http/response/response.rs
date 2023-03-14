use http::{StatusCode};

pub struct HttpResponse {
    status_code: StatusCode,
}

impl HttpResponse {
    pub fn new(status_code: StatusCode) -> Self {
        HttpResponse {
            status_code: status_code
        }
    }

    pub fn build(self) -> String {
        match self.status_code {
            StatusCode::OK => self.ok(),
            _ => self.bad_request()
        }
    }

    fn ok(self) -> String {
        String::from("HTTP/1.1 200 OK\r\n\r\n")
    }

    fn bad_request(self) -> String {
        String::from("HTTP/1.1 400 Bad Request\r\n\r\n")
    }
}