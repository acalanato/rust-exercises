use std::io::{BufRead, BufReader};
use std::path::Path;
use std::io::prelude::Read;
use std::fs::File;
use std::fs;
// follow from https://doc.rust-lang.org/std/io/



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
fn read_available(buf: &mut BufReader<File>) -> Box<[u8]> {
    let buf_slice = buf
        .fill_buf()
         // retorna o conteúdo do buffer e lê mais caso teja disponível.
        .expect("failed to read available bytes.")
        .into();
    // avisa pro leitor quantos bytes foram lidos pra não serem repetidos.
    buf.consume(buf.buffer().len());

    buf_slice
}

fn read_js(path: &Path) -> std::io::Result<()> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    assert_eq!(contents, "Hello, world!");
    Ok(())
}

fn read_only(path: &Path) -> std::io::Result<()> {
    let mut f = File::open(path)?;
    let mut data = vec![];
    f.read_to_end(&mut data)?;
    Ok(())
}

fn main() {

    let _default = Path::new("/dev/input/js0"); //funfa
    let _udev_ = Path::new("/dev/input/event17"); //existe, mas nem encontra
    let _test = Path::new("/home/vagner/out.txt"); //funfa com qq arquivo
    
//    let file = fs::OpenOptions::
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
        let buf_slice = read_available(&mut _js2);

        println!("{buf_slice:?}");
    }
*/
    let buffer = read_only(_test)
        .ok()
        .expect("Not there");
    println!("{:?}", buffer);
}
