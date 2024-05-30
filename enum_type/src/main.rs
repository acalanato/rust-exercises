#[derive(Debug)]
#[allow(dead_code)]
enum IpAddr {
    V4(String),
    V6(String),
}

#[derive(Debug)]
#[allow(dead_code)]
enum IpAddr2 {
    V4(u8, u8, u8, u8),
    V6(String),
}

#[allow(dead_code)]
enum Message {
    Quit,
    Move { x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}


#[derive(Debug)]
#[allow(dead_code)]
enum EstadoBr {
    Acre,
    Ceará,
    Bahia,
    SãoPaulo(CidadeBr),
}

#[derive(Debug)]
#[allow(dead_code)]
enum CidadeBr {
    Atibaia,
    Bragança,
    Campinas,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

#[allow(dead_code)]
fn estado_cidade(cidade: EstadoBr) -> String {
    match  cidade {
        EstadoBr::Acre => String::from("Bujari"),
        EstadoBr::Bahia => String::from("Amargosa"),
        EstadoBr::Ceará => String::from("Cascavel"),
        EstadoBr::SãoPaulo(estado) => {
            println!("Estado de São Paulo:{:?}", estado);
            String::from("Atibaia")
        }
    }
}

/*
enum Option<T> {
    None,
    Some(T),
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}


let home = IpAddr {
    kind: IpAddrKind::V4,
    address: String::from("127.0.0.1"),
};

let loopback = IpAddr {
    kind: IpAddrKind::V6,
    address: String::from("::1"),
};
*/

fn main() {
    let home = IpAddr::V4(String::from("127.0.0.1")); // same code as above
    let loopback = IpAddr::V6(String::from("::1"));

    println!("Home address is: {:?} and loopback: {:?}", home, loopback);

    // defined data types
    let home2 = IpAddr2::V4(127,0,0,1);
    let loopback2 = IpAddr2::V6(String::from("::1"));

    println!("Home address is: {:?} and loopback: {:?}", home2, loopback2);

    let some_number = Some(5);
    let some_char = Some('e');
    let some_string = Some(String::from("Easypeasy"));
    let absent_value: Option<i32> = None;

    
    print!("
Some number:\t{:?}
Some char:\t{:?}
some string:\t{:?}
Not null:\t{:?}\n",
           some_number,
           some_char,
           some_string,
           absent_value
    );

    print!("
A penny:\t{}
A nickel:\t{}
A dime: \t{}
One quarter:\t{}\n",
           value_in_cents(Coin::Penny),
           value_in_cents(Coin::Nickel),
           value_in_cents(Coin::Dime),
           value_in_cents(Coin::Quarter)

    );
    let cidade = estado_cidade(EstadoBr::SãoPaulo(CidadeBr::Atibaia));
    println!("{}", cidade)
}
