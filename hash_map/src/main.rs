use std::collections::HashMap;

fn main() {
    let mut stats = HashMap::new();
    stats.insert(String::from("Str"), 10);
    stats.insert(String::from("Dex"), 12);
    stats.insert(String::from("Vit"), 8);
    
    println!("Str: {}, Dex: {}, Vit: {}",
             stats.get("Str").copied().unwrap(),
             stats.get("Dex").copied().unwrap(),
             stats.get("Vit").copied().unwrap());
    // almost same as above code
    for (key, value) in &stats {
        println!("{key}: {value}")
    }
    //overwriting values
    stats.insert(String::from("Str"), 15);
    //only overwrite if it doesn't exist
    stats.entry(String::from("Str")).or_insert(14);
}
