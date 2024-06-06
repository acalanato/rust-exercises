use game_loop::question::{Asking, Asking2};
//pub mod quest1; //lives within project root


fn main() {
    game_loop::question::between_two(String::from("viver"), String::from("morrer"));
    game_loop::question::gen_choice(Asking{ask: String::from("Olá"),
                                           right: String::from("Tudo bem?"),
                                           wrong: String::from("Ouví alguém me chamar")});
    //    game_loop::question::gen_choice2(Asking2::Ask(String::from("Teste")), Asking2::Right(String::from(" ")),);
    //doesn't work
}

