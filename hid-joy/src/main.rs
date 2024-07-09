use hidraw::Device;
//use std::path::Path;
//use std::result::Result;


fn main() {
    let mut js0 = Device::open("/dev/hidraw0")
        .ok()
        .expect("Couldn't read device");
    println!("{:#?}", js0.get_raw_info());
}
