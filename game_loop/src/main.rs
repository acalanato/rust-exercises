//use crate::quest1::choice::Question; //lives within module quest1
pub mod quest1; //lives within project root

fn main() {
//    game_loop::choice::gen_choice(Question);
    game_loop::choice::between_two(String::from("viver"), String::from("morrer"));
}

