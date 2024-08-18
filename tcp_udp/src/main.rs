use std::{io::Read, net::{TcpListener, TcpStream}};
use std::result::Result;

/*
fn send(mut stream: TcpStream) -> Result<()>{
    stream = TcpStream::connect("10.0.1.1:80")?;
    let mut listen = String::new();
    stream.read_to_string(&mut listen)?;
    Ok(())
}
*/

fn receive(stream: TcpListener)  {
    println!("Listening on address {stream:?}");
    match stream.accept() {
	Ok((_socket, addr)) => println!("funfou: {addr:?}"),
	Err(e) => println!("não funfou: {e:?}"),
    }
    
}

fn main() {
    let stream = TcpListener::bind("127.0.0.1:80").unwrap();
    receive(stream);
    println!("Hello, world!");
}
