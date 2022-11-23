use std::net::TcpStream;
use std::io::prelude::*;

const IP: &str = "127.0.0.1:2000";

fn main() {
    let mut stream = TcpStream::connect(IP).unwrap();
    let mut buf = [0; 1024];

    stream.write("hello world\nI am messaging from rust\n\n".as_bytes());
    stream.read(&mut buf);
    let data = String::from_utf8(buf.to_vec()).unwrap();

    println!("Received: {}", data);
}
