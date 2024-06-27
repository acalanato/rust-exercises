use std::collections::HashMap;
use std::fs::read_to_string;

fn how_many_words(f: &str) -> u32 {
    let file = read_to_string(f)
        .ok()
        .expect("Deu ruim");
    let mut map = HashMap::new();

    for word in file.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    return map.len() as u32
}

fn main() {
    println!("Input file has {} words.", how_many_words("file.txt"));
}
