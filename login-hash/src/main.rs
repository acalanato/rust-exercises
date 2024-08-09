use std::io::{self, Result, Write};
use std::fs::{File, read_to_string};
use std::hash::{DefaultHasher, Hash, Hasher};
#[allow(unused_variables)]

#[derive(Hash)]
struct User {
    user: String,
    passwd: String,
    level: u8,
}

fn create_user(us: String, pas: String) -> Result<()>{
    let mut file = File::create("shadow")?;
    let user = User {
        user: us,
        passwd: pas,
	level: 1, // not used for now
    };
    let mut u = DefaultHasher::new();
    user.hash(&mut u);
    let hash = u.finish();
    file.write(hash.to_string().as_bytes())?;
    Ok(())
}

fn login() {
    let mut user = String::new();
    let mut passwd = String::new();
    let hash = read_to_string("shadow".to_string()).ok().expect("");
    let mut u = DefaultHasher::new();

    'login: loop {
        'user: loop {
            println!("User:");
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
	let user = User {
            user: user.clone(),
            passwd: passwd.clone(),
	    level: 1, // not used for now
	};
	user.hash(&mut u);
	let user = u.finish().to_string();
	if user == hash {
	    println!("Sucess!");
            break 'login;
	} else {
	    println!("User or password is invalid")
	}
    }

//    println!("\nLogin =\t\t{}Password =\t{}", user, passwd);
//    println!("Sucess!")
}

fn main() {
    println!("
What to do?
1 -> Login
2 -> Create Account
3 -> Exit");
    let mut opt = String::new();
    'menu: loop {
	
	io::stdin()
	    .read_line(&mut opt)
	    .expect("Invalid option");
	let choice: i32 = match opt.trim().parse(){
	    Ok(opt) => opt,
	    Err(_) => continue,
	};
	match choice {
	    1 => {login();
		  break},
	    
	    2 => {let mut user = String::new();
		  let mut passwd = String::new();
		  println!("Create new user");
		  io::stdin()
		  .read_line(&mut user)
		  .expect("Invalid user");
		  match user.trim().parse::<String>() {
		      Ok(user) => user,
		      Err(_) => continue,
		  };
		  println!("Password for the new account");
		  io::stdin()
		  .read_line(&mut passwd)
		  .expect("Invalid password");
		  match user.trim().parse::<String>() {
		      Ok(passwd) => passwd,
		      Err(_) => continue,
		  };
		  create_user(user, passwd)
		  .ok().expect("Failed to create user");
		  println!("User created!");
		  break 'menu},

	    3 => {println!("Ciao babe");
		  break 'menu},

	    _ => continue,
	    }
	};

}
