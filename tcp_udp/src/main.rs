#[allow(unused)]
use std::{io::{Read, Result}, net::{TcpListener, TcpStream}, env};


fn send(mut stream: TcpStream) -> Result<()>{
    stream = TcpStream::connect("10.0.1.1:80")?;
    let mut listen = String::new();
    stream.read_to_string(&mut listen)?;
    Ok(())
}


fn listen(stream: TcpListener)  {
    println!("Listening on address {stream:?}");
    match stream.accept() {
	Ok((_socket, addr)) => println!("funfou: {addr:?}"),
	Err(e) => println!("não funfou: {e:?}"),
    }
}

fn peer() -> Result<()>{
    todo!();
    Ok(());
}


fn main() {
    let args: Vec<String> = env::args().collect();
    let localhost = "127.0.0.1:80";
    let remote = peer();
    match args {
	send => {
	    send(remote);
	}
	receive => {
	    let stream = TcpListener::bind(localhost).unwrap();
	    listen(stream);
	}
    }
    println!("Sucess!");
}
