use std::collections::HashMap;
use std::fs::read_to_string;
use std::ops::Range;
use rand::{Rng, SeedableRng};

const FILE: &str = "file.txt";

fn _how_many_words(f: &str) -> usize {
    let file = read_to_string(f)
        .ok()
        .expect("Deu ruim");
    let mut map = HashMap::new();

    for word in file.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    return map.len() as usize
}


fn get_rand_word() -> String{
    let words_n = _how_many_words(FILE);
    let choice = rand::thread_rng().gen_range(0..words_n);
    let word_list: Vec<String> = read_to_string(FILE)
        .unwrap()
        .lines()
        .map(String::from)
        .collect();
    word_list[choice].to_string()
}

fn main() {
    println!("{}", get_rand_word())
}
