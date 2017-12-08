
extern crate rand;
extern crate swim;

use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::fs::File;
use rand::{thread_rng, Rng};


fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    let lanes = 10;
    let pool = swim::Pool::new(lanes);
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        let lane_id = thread_rng().gen_range(0, lanes);
        pool.send(lane_id, || {
            handle_connection(stream)
        });
    }

    fn handle_connection(mut stream: TcpStream) {
        let mut buffer = [0; 512];
        stream.read(&mut buffer).unwrap();
        println!("{}", String::from_utf8_lossy(&buffer[..]));
        let mut file = File::open("hello.html").unwrap();

        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", contents);

        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}
