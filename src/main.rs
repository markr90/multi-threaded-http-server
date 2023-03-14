use srv_http::server::{HttpServer};
pub mod srv_http;

fn main() {
    HttpServer::new("AppTest")
        .bind("127.0.0.1:3001")
        .run();
}

// fn handle_connection(mut stream: TcpStream) {
//     let buf_reader = BufReader::new(&mut stream);
//     let _http_request: Vec<_> = buf_reader
//         .lines()
//         .map(|result| result.unwrap())
//         .take_while(|line| !line.is_empty())
//         .collect();

//     let status_line = "HTTP/1.1 200 OK";
//     let contents = fs::read_to_string("public/index.html").unwrap();
//     let length = contents.len();

//     let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

//     stream.write_all(response.as_bytes()).unwrap();
// }
