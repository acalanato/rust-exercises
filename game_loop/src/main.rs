//use game_loop::question::{self, question_packer, Asking, Asking2, Questions};
mod quest1; //lives within project root

use crate::quest1::question::between_two;
//use crate::quest1::question::gen_choice2;

use game_loop::quest1::question::Asking;


fn main() {
    
    between_two(String::from("viver"), String::from("morrer"));
    Asking{ask: String::from("Olá"),
           right: String::from("Tudo bem?"),
           wrong: String::from("Ouví alguém me chamar")};
//    gen_choice2(Questions(String::from("a"), String::from("b"), String::from("c")));
    
}

