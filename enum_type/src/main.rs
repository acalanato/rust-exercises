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
    )
}
