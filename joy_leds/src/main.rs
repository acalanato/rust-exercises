use std::{ffi::OsString, fs::read_dir};
//use std::io::Error;

struct Leds {
    red: OsString,
    blue: OsString,
    green: OsString,
    alpha: OsString
}

fn get_leds() -> Leds {
    let list = read_dir("/sys/class/leds/").unwrap();

    let mut leds: Leds  = Leds {
	red: "".into(),
	blue: "".into(),
	green: "".into(),
	alpha: "".into()
    };

    for i in list {
	//	println!("{:?}", i.unwrap().file_name())
	match i.unwrap().file_name().to_str() {
	    Some(":red") => leds.red = OsString::from("input51:red"),
	    Some(":green") => leds.green = OsString::from("input51:green"),
	    Some(":blue") => leds.blue = OsString::from("input51:blue"),
	    Some(":global") => leds.alpha = OsString::from("input51:global"),
	    _ => panic!("No match!")
	}
    };
    leds
}

fn main() {

    get_leds();

    println!("Still works!")
}
