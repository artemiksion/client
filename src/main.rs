use std::net::{TcpStream, TcpListener};
use std::io::prelude::*;


fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:7878").unwrap();

    let buf :String = String::from("Hello someboddy");

    stream.write(&buf.as_bytes()).unwrap();
}
