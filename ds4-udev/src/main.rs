extern crate udev;
use udev::*;
use libc;
use std::io::BufReader;
use std::path::Path;
use std::io::prelude::Read;
use std::io::{self, BufRead};
use std::fs::File;
use std::fs;

/*
fn _udev_read(path: &Path) -> Result<()> {
    let dev = Device::from_syspath(path)?;
//    Device::properties(&dev);
}
 */

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

fn _buffered_read(path: &Path) -> std::io::Result<()> {
    let file = File::open(path)?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    assert_eq!(contents, "Hello, world!");
    Ok(())
}

//this fn is supposed to create a device
fn _create_udev() -> std::io::Result<()> {
    use std::{env, fs, os::linux::fs::MetadataExt};
    let args: Vec<String> = env::args().collect();
    let path = args.get(1).expect("No filename given");
    let metadata = fs::metadata(path).unwrap_or_else(|_| panic!("Can't open file: {}", path));
    let devtype = match metadata.st_mode() & libc::S_IFMT {
        libc::S_IFCHR => Some(DeviceType::Character),
        libc::S_IFBLK => Some(DeviceType::Block),
        _ => None,
    }.expect("Not a character or block special file");
    let ud = udev::Device::from_devnum(devtype, metadata.st_rdev())
        .expect("Couldn't construct udev from supplied path");
    println!("syspath of {} is {:?}", path, ud.syspath());
    let dn = ud.devnum();
    println!("devnum: {}", dn.unwrap());
    Ok(())
}

fn _read_js(path: &Path) -> std::io::Result<()> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    assert_eq!(contents, "Hello, world!");
    Ok(())
}

fn _read_only(path: &Path) -> std::io::Result<()> {
    let mut f = File::open(path)?;
    let mut data = vec![];
    f.read_to_end(&mut data)?;
    Ok(())
}

fn _read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn _cat(path: &Path) -> io::Result<String> {
    let mut f = File::open(path)?;
    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

//read everything!! must filter
fn _read_udev() -> io::Result<()> {
    let mut enumerator = udev::Enumerator::new()?;
    for device in enumerator.scan_devices()? {
        println!();
        println!("{:#?}", device);

//        println!("  [properties]");
//        for property in device.properties() {
//            println!("    - {:?} {:?}", property.name(), property.value());
//        }
        
//        println!("  [attributes]");
//        for attribute in device.attributes() {
//            println!("    - {:?} {:?}", attribute.name(), attribute.value());
//        }
    }
    Ok(())
}

fn _device(_path: &Path) -> io::Result<()>{
    let mut enumerator = udev::Enumerator::new()?;
    let device = enumerator.match_sysname("js0")?;
    println!("{:?}", device);
    Ok(())
}

fn _mon() -> io::Result<()> {
    let socket = udev::MonitorBuilder::new()?
        // .match_subsystem_devtype("usb", "usb_device")?
        .match_subsystem_devtype("usb", "input")?
        .listen()?;
    println!("{:#?}", socket.into_raw());
    Ok(())
}



fn main() {
    
    let _default = Path::new("/dev/input/js0"); //funfa
    let _udev_ = Path::new("/dev/input/event17"); //existe, mas nem encontra
    let _test = Path::new("/home/vagner/out.txt"); //funfa com qq arquivo
/*    
    let mut _js0: Vec<u8> = fs::read(_default)
        .ok()
        .expect("Can't find joystick");
    
    let mut _js2 = fs::OpenOptions::new()
        .read(true)
        .open("/dev/input/mice")
        .map(BufReader::new)
        .expect("Aint found shit");

    let buffer = read_only(_test)
        .ok()
        .expect("Not there");
    println!("{:?}", buffer);
    
    let buff = _cat(_default)
        .ok().
        expect("No joystick");
    println!("{}",buff)

    _device(_udev_) //invalid argument?

        _read_udev()
        .ok()
        .expect("Figure it out loser");
    _device(_default).ok().expect("Not found")
     */
    _mon()
        .ok();
}
