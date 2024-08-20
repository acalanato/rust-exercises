#[allow(unused)]
use std::{io::{Read, Result}, net::{TcpListener, TcpStream, UdpSocket}, env};

const LOCALHOST: &str = "127.0.1.1";
const PORT: &str = "789";

fn _send(mut stream: TcpStream) -> Result<()>{
    //stream = TcpStream::connect("10.0.1.1:80")?;
    let mut listen = String::new();
    stream.read_to_string(&mut listen)?;
    Ok(())
}


fn _listen(stream: TcpListener)  {
    println!("Listening on address {stream:?}");
    match stream.accept() {
	Ok((_socket, addr)) => println!("funfou: {addr:?}"),
	Err(e) => println!("não funfou: {e:?}"),
    }
}

fn _peer() -> Result<()>{
    let localhost = format!("{LOCALHOST}:{PORT}");
    let socket = UdpSocket::bind(localhost)?;
    socket.peer_addr()?;
    Ok(())
}



fn main() {
    let args: Vec<String> = env::args().collect();
    let localhost = format!("{LOCALHOST}:{PORT}");
    let remote = _peer();
/*
    match args {
	send => {
	    send(remote);
	}
	receive => {
	    let stream = TcpListener::bind(localhost).unwrap();
	    listen(stream);
	}
    }
*/
    println!("{localhost:?}");
    println!("{remote:?}");
}
