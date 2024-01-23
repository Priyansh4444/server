use std::net::{TcpListener, TcpStream};
use std::io::prelude::*;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    for stream in listener.incoming(){
        let stream = stream.unwrap();

        handle_connection(stream);
        println!("Connection Established!");
    }
}

fn handle_connection(mut stream: TcpStream){
    let mut buffer =  [0; 1024];
    stream.read(&mut buffer).unwrap();
    // Reading modifies internal stream so we send a mutable reference
    // Unwrap so if error panic!
    // println!(
    //     "Request {}",
    //     String::from_utf8_lossy(&buffer[..])
    //     // slice of bytes become a string including invalid characters
    // )
    
}