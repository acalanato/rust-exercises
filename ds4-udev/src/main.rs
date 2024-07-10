use udev::*;
use libc;
use std::io::BufReader;
use std::path::Path;
use std::io::prelude::Read;
use std::io::{self, BufRead};
use std::fs::File;
use std::fs;
// follow from https://doc.rust-lang.org/std/io/
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

//função de Mai
fn _read_available(buf: &mut BufReader<File>) -> Box<[u8]> {
    let buf_slice = buf
        .fill_buf()
         // retorna o conteúdo do buffer e lê mais caso teja disponível.
        .expect("failed to read available bytes.")
        .into();
    // avisa pro leitor quantos bytes foram lidos pra não serem repetidos.
    buf.consume(buf.buffer().len());

    buf_slice
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

//udev maybe?
fn _read_udev() -> io::Result<()> {
    let mut enumerator = udev::Enumerator::new()?;

    for device in enumerator.scan_devices()? {
        println!();
        println!("{:#?}", device);

        println!("  [properties]");
        for property in device.properties() {
            println!("    - {:?} {:?}", property.name(), property.value());
        }

        println!("  [attributes]");
        for attribute in device.attributes() {
            println!("    - {:?} {:?}", attribute.name(), attribute.value());
        }
    }

    Ok(())
}

fn main() {
    
    let _default = Path::new("/dev/input/js0"); //funfa
    let _udev_ = Path::new("/dev/input/event17"); //existe, mas nem encontra
    let _test = Path::new("/home/vagner/out.txt"); //funfa com qq arquivo
    
    //let file = fs::OpenOptions::
/*
        let mut mice = OpenOptions::new()
        .read(true)
        .open("/dev/input/mice")
        // pra poder ler o conteúdo do File sequencialmente quando tiver disponível.
        .map(BufReader::new)
        .expect("can't open device.");
*/
    
    let mut _js0: Vec<u8> = fs::read(_default)
        .ok()
        .expect("Can't find joystick");
    
    let mut _js2 = fs::OpenOptions::new()
        .read(true)
        .open("/dev/input/mice")
        .map(BufReader::new)
        .expect("Aint found shit");
/*    
    loop {
        let buf_slice = _read_available(&mut _js2);

        println!("{buf_slice:?}");
    }

    let buffer = read_only(_test)
        .ok()
        .expect("Not there");
    println!("{:?}", buffer);
    
    if let Ok(lines) = _read_lines(_default) {
        for line in lines.flatten() {
            println!("{}", line);
        }
    }

    let buff = cat(_udev_).ok().expect("No joystick");
    println!("{}",buff)
*/
    _read_udev()
        .ok()
        .expect("Figure it out loser");
    
    println!("{:#?}", _read_udev())
}
