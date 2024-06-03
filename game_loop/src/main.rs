//use rand::Rng;
use std::io;

fn main() {
    pub fn vida() {
        loop {
            let mut choice = String::new();
            println!("Escolha uma opção:\n1-) Viver\n2-) Morrer");
            io::stdin().read_line(&mut choice).expect("Tente novamente");
            let choice: i32 = match  choice.trim().parse() {
                Ok(choice) => choice,
                Err(_) => continue,
            };
            match choice {
                1 => {println!("Você escolheu viver");  continue},
                _ => {println!("Você escolheu se juntar ao Olavo"); break},
            }
        }
    }
}

