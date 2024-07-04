// follow from https://doc.rust-lang.org/std/io/
use std::fs;

#[allow(dead_code)]
struct JsEvent {
    time: u32,
    value: u16,
    t: u8,
    key: u8,
}

#[allow(dead_code)]
struct InputEvent {
        timeval: f32,
        t: u8,
        code: u8,
        value: u32,
}


//impl Read for JsEvent {
//    fn read_exact(&mut self, buf: &mut [u8]) -> io::Result<()> {
//        self.read(buf)?;
//    }
//}

fn main() {

    let _default = "/dev/input/mouse0"; //funfa
    let _udev_ = "/dev/input/event17"; //existe, mas nem encontra
    let _test = "/home/vagner/out.txt"; //funfa com qq arquivo
    
    let js0: Vec<u8> = fs::read(_default)
        .ok()
        .expect("Can't find joystick");

    
    println!("{:?}", js0);
}
