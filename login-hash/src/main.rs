use std::io;

fn main() {
    let mut user = String::new();
    let mut passwd = String::new();

    'login: loop {
        'user: loop {
            println!("User;");
            //let mut user = String::new();
            io::stdin()
                .read_line(&mut user)
                .expect("Invalid user");
            let user: String = match user.trim().parse::<String>() {
                Ok(username) => {username; break 'user},
                Err(_) => continue,
            };
        }
        'pass: loop {
            println!("Password;");
            io::stdin()
                .read_line(&mut passwd)
                .expect("Invalid user");
            let passwd: String = match passwd.trim().parse::<String>() {
                Ok(passwd) => {passwd; break 'pass},
                Err(_) => continue,
            };
        }
        break 'login;
    }
    println!("Login =\t\t{}\nPassword =\t{}", user, passwd);
    println!("Sucess!")
}