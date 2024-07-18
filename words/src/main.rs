use std::collections::HashMap;
use std::fs::read_to_string;
use rand::Rng;

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

fn _get_rand_word() -> String{
    let word_list: Vec<String> = read_to_string(FILE)
        .unwrap()
        .lines()
        .map(String::from)
        .collect();
    let words_n = word_list.len();
    let choice = rand::thread_rng().gen_range(0..words_n);
    word_list[choice].to_string()
}


fn word_filter(list: Vec<String>) -> Vec<String>{
    let mut result = Vec::new();
    for word in list {
        if word.contains("a"){
            result.push(word)
        }
    }
    result
}


fn _contains<'a>(s: &'a str, c: &'a str) -> String {
    if s.contains(c) {
        s.to_string()
    } else {
        //"does not contain".to_string()
        format!("\"{}\" does not contain \"{}\"", s, c)
    }
}

fn _list_words() -> Vec<String>{
        read_to_string(FILE)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

fn main() {
    let _list_words = _list_words();
    //println!("{}", contains(&get_rand_word(), "a"))
    println!("before:\t {}\nafter:\t {}", _list_words.len(), word_filter(_list_words).len())
}
