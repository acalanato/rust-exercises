use std::fs::File;
use std::{fs, io};
use std::io::{ErrorKind, Read};
use std::error::Error;

fn read_from_f () -> String {
    let file_path = "my_file.txt";
    let my_file_result = File::open(file_path);

    let _my_file = match my_file_result {
        Ok(file) => {println!("{:?}",file);
                     return fs::read_to_string(file_path).expect("fdp")},
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create(file_path) {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating file: {e:?}"),
            },
            other_error => {
                panic!("Other errors: {other_error:?}")
            }
        },
                
    };
    return "You must mess up real bad to see this message".to_string();
}

fn _read_file_error() -> Result<String, io::Error> {
    let username_file_result = File::open("not_present.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();
    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

fn _read_username_from_file() -> Result<String, io::Error> {
    let mut username_file = File::open("not_here.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

fn _shorter_error() -> Result<String, io::Error> {

    let mut username = String::new();
    File::open("not_here_too.txt")?.read_to_string(&mut username)?;
    Ok(username)
}

fn _shortest() -> Result<String, io::Error> {
    fs::read_to_string("not_here")
}

/* < Doesn't work for reading to string >
fn read_f_too () -> String {
    let file_path = "my_file.txt";
    let my_file = fs::read_to_string(file_path).unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("my_file.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {error:?}");
            })
        } else {
            panic!("Problem opening the file: {error:?}");
        }
    });
    return "".to_string()
}
*/
fn main() -> Result<(), Box<dyn Error>> {
    //    panic!("crash with success!");
    //    let v = vec![3, 2, 1];
    //    v[10];
    let s = read_from_f();
    println!("{}", s);

    
    let _greeting_file = File::open("missing_file.txt")
        .expect("file not inclued in project");

    let _greeting = File::open("Hell_no.txt")?;
    Ok(())
    //println!("add RUST_BACKTRACE=1 to env in order to backtrace");
}
