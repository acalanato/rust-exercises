// follow from https://doc.rust-lang.org/std/io/
use std::{fs::File, io::{self, Read}};

    struct JsEvent {
        time: u32,
        value: u16,
        t: u8,
        key: u8,
    }

impl Read for JsEvent {
    fn read_exact(&mut self, buf: &mut [u8]) -> io::Result<()> {
        self.read(buf)?;
    }
}

fn main() {

    
    let mut js0 = File::open("/dev/input/js0")?;
//        .ok()
//        .expect("Can't find joystick");

    let JsEvent {
        time: js0.
    }
    
    println!("{:?}", file);
}
