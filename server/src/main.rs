use std::{
    fs, io::{BufRead, BufReader, Write}, net::TcpListener
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();

        println!("{:?}", stream);

        let buf_reader: BufReader<&std::net::TcpStream> = BufReader::new(&stream);

        let http_req: Vec<_> = buf_reader
            .lines()
            .map(|result| result.unwrap())
            .take_while(|line| !line.is_empty())
            .collect();

        println!("Request : {:?} ", http_req);

        let status_line = "HTTP/1.1 200 OK\r\n\r\n";
        let contents =fs::read_to_string("res.html").unwrap();
         let length = contents.len();
         let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
        stream.write_all(response.as_bytes()).unwrap();
    }
}
