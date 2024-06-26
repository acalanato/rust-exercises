use std::collections::HashMap;
use std::fs::read_to_string;

fn how_many_words(file: &str)  {
//    let file = "hello world wonderful world";
    let f = read_to_string(file)
        .ok()
        .expect("Couldn't read file");
    let mut map = HashMap::new();

    for word in f.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
//    return map
    println!("{map:?}");
}

fn main() {
    how_many_words("file.txt");
}
