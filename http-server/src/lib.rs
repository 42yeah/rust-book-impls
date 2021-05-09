use std::{io::{Read, Write}, net::TcpStream, thread, time::Duration};


pub fn create_response(status: i32, filename: &str) -> String {
    let header = match status {
        200 => "HTTP/1.1 200 OK",
        404 => "HTTP/1.1 404 NOT FOUND",
        _ => unimplemented!("Not implemented")
    };
    let content = std::fs::read_to_string(filename).unwrap();
    format!("{}\r\nContent-Length: {}\r\n\r\n{}", header,
        content.len(), content)
}

pub fn handle_request(req: &str) -> String {
    let (code, filename) = if req.starts_with("GET / HTTP/1.1") {
        (200, "index.html")
    } else if req.starts_with("GET /sleep HTTP/1.1") {
        thread::sleep(Duration::from_secs(5));
        (200, "index.html")
    } else {
        (404, "404.html")
    };
    create_response(code, filename)
}

pub fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    let request = String::from_utf8_lossy(&buffer);
    stream.write(handle_request(&request).as_bytes()).unwrap();
    stream.flush().unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_response() {
        let content = std::fs::read_to_string("index.html").unwrap();
        assert_eq!(handle_request("\
GET / HTTP/1.1
Host: 127.0.0.1:7878
Connection: close
"), format!("HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}", content.len(), content));
    }

    #[test]
    fn test_404() {
        let content = std::fs::read_to_string("404.html").unwrap();
        assert_eq!(handle_request("\
GET /404 HTTP/1.1
Host: 127.0.0.1:7878
Connection: close
"), format!("HTTP/1.1 404 NOT FOUND\r\nContent-Length: {}\r\n\r\n{}", content.len(), content));
    }
}
