use std::io::{BufReader, prelude::*};
use std::net::{TcpListener, TcpStream};

fn main() {
    let listener: TcpListener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream: std::net::TcpStream = stream.unwrap();

        println!("Connection established")
    }
}

//By defualt function parameters are immutable , we need to make the parameters mutable explicitly
fn handle_connection(mut stream: TcpStream) {
    let buf_reader: BufReader<&TcpStream> = BufReader::new(&stream);

    let http_req: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    println!("Connection Established Successfully")
}
