//use core::str;


fn _disemvowel(s: &str) -> String{ //from my stuff
    let mut out = vec![];
    let list = s.chars().map(|x| x as u8).collect::<Vec<u8>>();
    
    for x in list.iter() {
        if "aeiou".as_bytes().contains(x) {out.push(*x)}
    }
    String::from_utf8(out).expect("")
}


fn disemvowel(s: &str) -> String{ //from rust-book
    s.replace(['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'], "")
    //rem(s)
}

/*
fn disemvowel(s: &str) -> String { top answer
    s.chars()
        .filter(|&c| !"aeiou".contains(c.to_ascii_lowercase()))
        .collect()
}
*/

fn main() {
    println!("{}", disemvowel("Remove stuff"))
    //println!("{}", disemvowel("Remove the vowels from this"));
}

#[cfg(test)]
mod tests {
    use super::disemvowel;
    
    #[test]
    fn example_test() {
        assert_eq!(disemvowel("This website is for losers LOL!"), "Ths wbst s fr lsrs LL!");
    }
}
