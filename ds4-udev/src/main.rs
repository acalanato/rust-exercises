use std::fs::read;

fn main() {
    struct Js_event {
        time: u32,
        value: u16,
        t: u8,
        key: u8,
    }
    let file = read("/dev/input/js0");
    println!("{:?}", file);
}
