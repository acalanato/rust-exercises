#[allow(unused)]
use std::{io::Read, net::{TcpListener, TcpStream}};
#[allow(unused)]
use std::result::Result;

/*
fn send(mut stream: TcpStream) -> Result<()>{
    stream = TcpStream::connect("10.0.1.1:80")?;
    let mut listen = String::new();
    stream.read_to_string(&mut listen)?;
    Ok(())
}
*/

fn listen(stream: TcpListener)  {
    struct Stream {addr: String, fd: i32,}
    //let Stream { addr: addr0 , fd: fd0 } = stream;
    //Listening on address TcpListener { addr: 127.0.0.1:80, fd: 3 }

    println!("Listening on address {stream:?}");
    match stream.accept() {
	Ok((_socket, addr)) => println!("funfou: {addr:?}"),
	Err(e) => println!("não funfou: {e:?}"),
    }
}

fn main() {
    let stream = TcpListener::bind("127.0.0.1:80").unwrap();
    listen(stream);
    println!("Sucess!");
}
