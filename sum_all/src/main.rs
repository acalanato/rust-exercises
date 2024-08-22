
fn summation(n: i32) -> i32 {
    let num = (0..=n).collect::<Vec<i32>>();
    num.iter().sum::<i32>()
}
fn main() {
    println!("{}!",summation(4))
}

#[cfg(test)]
mod tests {
    use super::summation;

    #[test]
    fn basic_tests() {
        assert_eq!(summation(1), 1);
        assert_eq!(summation(8), 36);
        assert_eq!(summation(22), 253);
        assert_eq!(summation(100), 5050);
        assert_eq!(summation(213), 22791);
    }
}
