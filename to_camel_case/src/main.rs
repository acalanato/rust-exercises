
#[allow(unused)]
fn to_camel_case(text: &str) -> String {
    let mut out = String::new();
    if text.len() > 1 {
	let first = &text[0..1];
	out = text.replace("-","_")
	    .split("_")
	    .map(|x| x[0..1].to_ascii_uppercase() + &x[1..] )
	    .collect();
	out = first.to_owned() + &out[1..];
    } else {
	out = text.chars().collect();
    }
    out
}

fn _to_camel_case(text: &str) -> String {
    let chars: Vec<_> = text.replace("-", "_").as_bytes().iter().map(|x| *x as u8).collect();
    let mut out = Vec::new();
    for (i,val) in chars.iter().enumerate() {
	if val == &b'_' {
	    out.push(chars[i+1].to_ascii_uppercase());
	} else {
	    out.push(*val);
	}
    }
    String::from_utf8(out).unwrap()
}

 /*
fn to_camel_case(text: &str) -> String {
    text.split(&['-', '_'])
        .enumerate()
        .map(|(i, w)| match i {
            0 => w.to_string(),
            _ => w[..1].to_uppercase() + &w[1..],
        })
        .collect()
}
 */



fn main() {
    println!("{}", to_camel_case("The_stealth_warrior"));
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
