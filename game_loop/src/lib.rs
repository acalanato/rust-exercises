pub mod question {
    use std::io;

    pub struct Asking {
        pub ask: String,
        pub right: String,
        pub wrong: String,
    }

    pub struct Questions(pub String, pub String, pub String);

/*
    pub fn question_packer (a: str, b:str, c: str) -> (String, String, String) {
        let tuple: (String, String, String) = (a, b, c);
        return tuple
    }
*/
    
    pub enum Asking2 {
        Ask(String),
        Right(String),
        Wrong(String),
    }
    
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
    
    pub fn gen_choice(question: Asking) {
        loop {
            let mut choice = String::new();
            print!("{}\n", question.ask);
            io::stdin().read_line(&mut choice).expect("Tente novamente");
            let choice: i32 = match  choice.trim().parse() {
                Ok(choice) => choice,
                Err(_) => continue,
            };
            match choice {
                1 => {println!("{}\n", question.right);  continue},
                _ => {println!("{}\n", question.wrong); break},
            }
        }
    }

    pub fn gen_choice2(q: Questions) {
        loop {
            let mut choice = String::new();
            print!("{}\n", q.0);
            io::stdin().read_line(&mut choice).expect("Tente novamente");
            let choice: i32 = match  choice.trim().parse() {
                Ok(choice) => choice,
                Err(_) => continue,
            };
            match choice {
                1 => {println!("{}\n", q.1);  continue},
                _ => {println!("{}\n", q.2); break},
            }
        }
    }
}
