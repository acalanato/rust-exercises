use std::io::Read;

fn main() {
    let test_a = digitize(349587);
//    let test_b = digitize(35231);
//    let test_c = digitize(0);
    println!("{test_a} {test_b} {test_c}");
}

fn digitize(n: u64) -> Vec<u8> {
    let s: String = n.to_string().chars().rev().collect();
    return Vec::from(s);
}

/*
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fixed() {
        assert_eq!(digitize(349587), vec![7,9,5,8,4,3]);
        assert_eq!(digitize(35231), vec![1,3,2,5,3]);
        assert_eq!(digitize(0), vec![0]);
    }
}
*/
