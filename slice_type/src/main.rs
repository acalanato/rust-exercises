fn main() {
    let s = String::from("hello words");

    let _word = first_word(&s);

//    s.clear(); // don't do that

    hello_slice();

    println!("the first word is: {}", _word);

    let _word = first_word(&s[0..6]);
    let _word = first_word(&s[..]);
    let _word = first_word(&s);
    let my_string_literal = "hello world";
    let _word = first_word(&my_string_literal[0..6]);
    let _word = first_word(&my_string_literal[..]);
    let _word = first_word(my_string_literal);
    println!("{_word}")
}

//fn first_word(s: &String) -> &str {
fn first_word(s: &str) -> &str {

    let bytes = s.as_bytes();

    for (i,&item) in bytes.iter().enumerate() {
	if item == b' ' {
	    return &s[0..i];
	}
    }
    &s[..]
}



fn _second_word(_s: &String) -> (usize, usize) {
    todo!("")
}

fn hello_slice() {
    let s = String::from("hello words");
    let mid = 5;
    let h = &s[..mid];
    let w = &s[mid+1..];
    println!("{} {}", h, w)
}
