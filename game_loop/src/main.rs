mod quest1; //lives within project root

//use crate::quest1::question::between_two;
use game_loop::quest1::question::between_multi;
use game_loop::quest1::question::between_two;
use game_loop::quest1::question::Asking;
use game_loop::quest1::question::Asking2;



fn main() {
    
//    between_two(String::from("viver"), String::from("morrer"));

    Asking{ask: String::from("Olá"),
           right: String::from("Tudo bem?"),
           wrong: String::from("Ouví alguém me chamar")};

    between_multi(Asking2{ask: String::from("Escolha um número entre 1 e 5"),
                          a: (String::from("Resposta errada"), String::from("Erraste"), false),
                          b: (String::from("Resposta errada"), String::from("Acertou krai!"), true),
                          c: (String::from("Resposta errada"), String::from("Errou"), false),
                          d: (String::from("Resposta errada"), String::from("Vish..."), false),
                          e: (String::from("Resposta errada"), String::from("Passou longe!"), false)}
    )
}
