use std::{io::Write, net::TcpListener};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8000").unwrap();

    for req in listener.incoming() {
        let mut req = req.unwrap();
        let req_addr = req.peer_addr().unwrap();

        println!("Req: {:?}\nAddr: {:?}", req, req_addr);

        let body = format!(r#"{{"Protocol": "HTTP/1.1","Your Addres is": "{req_addr}"}}"#);

        let response = format!(
            "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nContent-Type: application/json\r\nConnection: close\r\n\r\n{}",
            body.len(),
            body
        );

        req.write_all(response.as_bytes()).unwrap();
    }
}
