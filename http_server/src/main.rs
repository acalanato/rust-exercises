#[allow(unused_imports)]
use std::net::{TcpListener, TcpStream};
use std::io::{prelude::*, BufReader};    

#[allow(unused)]
fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    let status_line = "HTTP/1.1 200 OK";
    let contents = std::fs::read_to_string("Default.html").unwrap();
    let length = contents.len();

    let response = format!("{status_line}\r\nCntent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap()
}


fn main() {
    println!("Logs from your program will appear here!");
    
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    //let listener = TcpListener::bind("192.168.0.109:4221").unwrap();

    for stream in listener.incoming() {
	match stream {
	    Ok(stream) => {handle_connection(stream)},
	    Err(e) => println!("fonfon: {e}"),
	}
    }
   
}
