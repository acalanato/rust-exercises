
fn to_alternating_case(s: &str) -> String { //my fn
    let mut out = Vec::new();
    for x in s.as_bytes() {
	if x.is_ascii_lowercase() {
	    out.push(x.to_ascii_uppercase())
	} else {
	    out.push(x.to_ascii_lowercase())
	}
    }
    String::from_utf8(out).expect("")
}

fn _to_alternating_case(s: &str) -> String { //top fn
    s.chars().map(|c| {
        match c {
            c if c.is_uppercase() => c.to_lowercase().next().unwrap(),
            c if c.is_lowercase() => c.to_uppercase().next().unwrap(),
            _ => c
        }
    }).collect()
}

/*
fn _to_alternating_case2(s: &str) -> String { //variation
  s.chars().map(|x| {
    match x {
      'a'...'z' => x.to_ascii_uppercase(),
      'A'...'Z' => x.to_ascii_lowercase(),
      _ => x,
    }
  }).collect()
}
*/

fn main() {
    to_alternating_case("Teste");
    println!("Sucess!");
}


#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn example_tests() {
    assert_eq!("HELLO WORLD", to_alternating_case("hello world"));
    assert_eq!("hello world", to_alternating_case("HELLO WORLD"));
    assert_eq!("HELLO world", to_alternating_case("hello WORLD"));
    assert_eq!("hEllO wOrld", to_alternating_case("HeLLo WoRLD"));
    assert_eq!("Hello World", to_alternating_case(&to_alternating_case("Hello World")[..]));
    assert_eq!("12345", to_alternating_case("12345"));
    assert_eq!("1A2B3C4D5E", to_alternating_case("1a2b3c4d5e"));
    assert_eq!("sTRING.tOaLTERNATINGcASE", to_alternating_case("String.ToAlternatingCase"));
  }
}
