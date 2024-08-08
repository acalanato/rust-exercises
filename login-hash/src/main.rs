use std::io::{self, Read, Result, Write};
use std::fs::File;
use std::hash::{DefaultHasher, Hash, Hasher};
#[allow(unused_variables)]

#[derive(Hash)]
struct User {
    user: String,
    passwd: String,
    level: u8,
}


fn _create_user(us: String, pas: String) -> Result<()>{
    let mut file = File::create("shadow")?;
    let user = User {
        user: us,
        passwd: pas,
	level: 1,
    };
    let mut u = DefaultHasher::new();
    user.hash(&mut u);
    let hash = u.finish();
    //file.write_all(b"{hash}")?;
    file.write(hash.to_string().as_bytes())?;
    Ok(())
}

 /*
fn hash_p<T: Hash>(t: &T) -> u64{
    let mut p = DefaultHasher::new();
    t.hash(&mut p);
    p.finish()
}
 */

fn _login() {
    let mut user = String::new();
    let mut passwd = String::new();
    
    'login: loop {
        'user: loop {
            println!("User:");
            //let mut user = String::new();
            io::stdin()
                .read_line(&mut user)
                .expect("Invalid user");
            match user.trim().parse::<String>() {
                Ok(username) => {drop(username); break 'user},
                Err(_) => continue,
            };
        }
        'pass: loop {
            println!("Password:");
            io::stdin()
                .read_line(&mut passwd)
                .expect("Invalid user");
            match passwd.trim().parse::<String>() {
                Ok(passwd) => {drop(passwd); break 'pass},
                Err(_) => continue,
            };
        }
        break 'login;
    }
    println!("\nLogin =\t\t{}Password =\t{}", user, passwd);
    println!("Sucess!")
}

fn main() {
    println!("
What to do?
1 -> Login
2 -> Create Account
3 -> Exit");
    let mut opt = String::new();
    'option: loop {
	io::stdin()
            .read_line(&mut opt)
            .expect("Invalid option");
	match opt.trim().parse::<String>(){
	    Ok(opt) => opt,
	    Err(_) => continue,
	};
	break 'option
    }
    loop {
	match opt.as_str() {
	    "1" => println!("opt1"),
	    "2" => println!("opt2"),
	    "3" => println!("opt3"),
	    _ => break,
	
	};
    }
    //create_user(us, pas);
    //login()
}
