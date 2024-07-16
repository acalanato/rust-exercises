use std::collections::HashMap;
use std::fs::read_to_string;
use rand::{Rng, SeedableRng, prelude};


const FILE: &str = "file.txt";
const N: usize = 64;

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

struct ARngSeed([u8; N]);
struct ARng(ARngSeed);

impl Default for ARngSeed {
    fn default() -> Self {
        ARngSeed([0; N])
    }
}

impl AsMut<[u8]> for ARngSeed{
    fn as_mut(&mut self) -> &mut [u8] {
        &mut self.0
    }
}

impl SeedableRng for ARng {
    type Seed = ARngSeed;
    fn from_seed(seed: Self::Seed) -> Self {
        ARng(seed)
    }
}

fn get_rand_word() -> String{
    let words_n = _how_many_words(FILE);
    let mut seed = ChaCha20Rng::from_entropy();
    
    rand::thread_rng().gen_range(0..=words_n);

    
    
    let word_list: Vec<String> = read_to_string(FILE)
        .unwrap()
        .lines()
        .map(String::from)
        .collect();
    word_list[words_n].to_string()
}

fn main() {
    println!("{}", get_rand_word())
}
