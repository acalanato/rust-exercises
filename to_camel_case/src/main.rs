
#[allow(unused)]
fn _to_camel_case(text: &str) -> String {
    let mut out = text.bytes().map(|x| x as u8).collect::<Vec<u8>>();
    for x in out {
	if x.is_ascii_punctuation() {
	    x.to_ascii_lowercase();
	}
    }
    "".to_string()
}

fn to_camel_case(text: &str) -> String {
    if text.len() > 1 {
	text.replace("-","_")
	    .split("_")
	    .map(|x| x[0..1].to_ascii_uppercase() + &x[1..] )
	    .collect()
    } else {
	text.chars().collect()
    }    
}


fn to_camel_case2(text: &str) -> String {
    let s = text.as_bytes().into_iter();
    for x in s {
	if s.next().eq(&Some(&b'_')) {x.to_ascii_uppercase()
	} else {continue};
    };
    String::from_utf8(s.as_slice().to_vec()).unwrap()
}
/*

x[0..1].make_ascii_uppercase();


let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
*/



fn main() {
    println!("{}", to_camel_case("the_stealth_warrior"));
    println!("Sucess!");
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::to_camel_case;
    
    const ERR_MSG: &str = "\nYour result (left) did not match the expected output (right)";
    
    fn dotest(s: &str, expected: &str) {
        assert_eq!(to_camel_case(s), expected, "{ERR_MSG} with text = \"{s}\"")
    }

    #[test]
    fn fixed_tests() {
        dotest("","");
        dotest("the_stealth_warrior", "theStealthWarrior");
        dotest("The-Stealth-Warrior", "TheStealthWarrior");
        dotest("A-B-C", "ABC");
    }
}
