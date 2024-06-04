pub mod choice {
    use std::io;
    pub fn between_two(opt1: String, opt2: String) {
        loop {
            let mut choice = String::new();
            println!("Escolha uma opção:\n1-) Viver\n2-) Morrer");
            io::stdin().read_line(&mut choice).expect("Tente novamente");
            let choice: i32 = match  choice.trim().parse() {
                Ok(choice) => choice,
                Err(_) => continue,
            };
            match choice {
                1 => {println!("Você escolheu {}", opt1);  continue},
                _ => {println!("Você escolheu {} e se juntar ao Olavo", opt2); break},
            }
        }
    }

    /* i want to feed a struct into this function
    pub fn gen_choice(Question) {
        loop {
            let mut choice = String::new();
            println!("Escolha uma opção:\n1-) Viver\n2-) Morrer");
            io::stdin().read_line(&mut choice).expect("Tente novamente");
            let choice: i32 = match  choice.trim().parse() {
                Ok(choice) => choice,
                Err(_) => continue,
            };
            match choice {
                1 => {println!("Você escolheu {}", opt1);  continue},
                _ => {println!("Você escolheu {} e se juntar ao Olavo", opt2); break},
            }
        }
    }
    */
}
