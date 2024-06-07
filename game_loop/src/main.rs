//use game_loop::question::{self, question_packer, Asking, Asking2, Questions};
//pub mod quest1; //lives within project root


fn main() {
    game_loop::question::between_two(String::from("viver"), String::from("morrer"));
    game_loop::question::gen_choice(Asking{ask: String::from("Olá"),
                                           right: String::from("Tudo bem?"),
                                           wrong: String::from("Ouví alguém me chamar")});
    game_loop::question::gen_choice2(Questions(String::from("a"), String::from("b"), String::from("c")));
    //doesn't work
}

