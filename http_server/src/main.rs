#[allow(unused_imports)]
use std::net::{TcpListener, TcpStream};
use std::io::{prelude::*, BufReader};

#[allow(unused)]
fn _handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);

    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();
    
    let response = "HTTP/1.1 200 OK\r\n\r\n";
    
    println!("Request: {http_request:#?}");

    stream.write_all(response.as_bytes()).unwrap()
}

#[allow(unused)]
fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    let status_line = "HTTP/1.1 200 OK";
    let contents = std::fs::read_to_string("SomeFile").unwrap();
    let length = contents.len();

    let response = format!("{status_line}\r\nCntent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap()
}


fn main() {
    println!("Logs from your program will appear here!");
    
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    /*
    for stream in listener.incoming() {
        let stream = stream.unwrap();
	
	handle_connection(stream);
    }
    
    for stream in listener.incoming() {
        match stream {
            Ok(_stream) => {
		for stream in listener.incoming() {
		    let stream = stream.unwrap();
		    println!("accepted new connection");
		    handle_connection(stream)
		}
            }
	    Err(e) => {
                println!("error: {}", e);
	    }
	}
    }
     */
    for stream in listener.incoming() {
	match stream {
	    Ok(stream) => {handle_connection(stream)},
	    Err(e) => println!("fonfon: {e}"),
	}
    }
   
}
