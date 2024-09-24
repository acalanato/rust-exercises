
fn solution(phrase: &str) -> String {
     String::from_utf8(phrase.bytes().into_iter().rev().collect()).unwrap()
}


fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sample_test() {
        assert_eq!(solution("world"), "dlrow");
    }
}
