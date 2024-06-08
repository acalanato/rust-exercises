pub mod question {
    use std::io;

    pub struct Asking {
        pub ask: String,
        pub right: String,
        pub wrong: String,
    }

    
    pub struct  Asking2 {
        pub ask: String,
        pub a: (String, String, bool),
        pub b: (String, String, bool),
        pub c: (String, String, bool),
        pub d: (String, String, bool),
        pub e: (String, String, bool),
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

        pub fn between_multi(opt: Asking2) {
        loop {
            let mut choice = String::new();
            println!("{}", opt.ask);
            io::stdin().read_line(&mut choice).expect("Tente novamente");
            let choice: i32 = match  choice.trim().parse() {
                Ok(choice) => choice,
                Err(_) => continue,
            };
            match choice {
                1 => {println!("{}", opt.a.1); if opt.a.2 == true {break}},
                2 => {println!("{}", opt.b.1); if opt.b.2 == true {break}},
                3 => {println!("{}", opt.c.1); if opt.c.2 == true {break}},
                4 => {println!("{}", opt.d.1); if opt.d.2 == true {break}},
                5 => {println!("{}", opt.e.1); if opt.e.2 == true {break}},
                _ => {println!("Invalid answer, try again!"); continue},
            }
        }
    }

}
