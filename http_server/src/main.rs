#[allow(unused_imports)]
use std::net::{TcpListener, TcpStream};
use std::io::{prelude::*, BufReader};


fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    let response = "HTTP/1.1 200 OK\r\n\r\n";

    stream.write_all(response.as_bytes()).unwrap();
    println!("Request: {http_request:#?}");
}


fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    let response = "HTTP/1.1 200 OK\r\n\r\n";
    
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    
    for stream in listener.incoming() {
        match stream {
            Ok(_stream) => {
                println!("accepted new connection");

		for stream in listener.incoming() {
		    let stream = stream.unwrap();

		    handle_connection(stream)
		}
		
            }
	    Err(e) => {
                println!("error: {}", e);
	    }
	}
    }
}
