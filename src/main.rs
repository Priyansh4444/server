use std::fs;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
        println!("Connection Established!");
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    // Reading modifies internal stream so we send a mutable reference
    // Unwrap so if error panic!
    // println!(
    //     "Request {}",
    //     String::from_utf8_lossy(&buffer[..])
    //     // slice of bytes become a string including invalid characters
    // )

    let get = b"GET / HTTP/1.1\r\n";

    let (status_line, filename) = 
    if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "index.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };
    let contents = fs::read_to_string(filename).unwrap();
    let length: usize = contents.len();
    
    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
    
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
    // flush will wait type all bytes are written to the connection
}
