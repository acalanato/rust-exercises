fn main() {
    let s1 = String::from("hello");
    let len = calculate_len(&s1); // press & to reference
    
    println!("The lenght of '{}' is {}", s1, len);
}

fn calculate_len(s: &String) -> usize {
    s.len()
} // s goes out of scope but doesn't drop because it isn't owned

fn change(s: &String) -> String {
    let out: &String = s.push_str(", world") as String;
//    return out
}
