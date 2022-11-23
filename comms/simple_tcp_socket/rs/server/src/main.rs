use std::net::{TcpListener, TcpStream};
use std::io::{prelude::*, BufReader};

const IP: &str = "127.0.0.1:2000";

fn main() {
    let listener = TcpListener::bind(IP).unwrap();
    println!("Hello, world!");

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        println!("connected {:?}", stream);
        handle_conn(stream);
    }
}

fn handle_conn(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let data: Vec<_>= buf_reader.lines().map(|line| line.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();
    println!("data is: {:#?}", data);

    let response = data.iter().map(|x| x.to_string()).collect::<String>();
    stream.write(response.as_bytes()).unwrap();
}
