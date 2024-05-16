

fn main() {
    let test_a = digitize(349587);
    let test_b = digitize(35231);
    let test_c = digitize(0);
    println!("{test_a:?}\n{test_b:?}\n{test_c:?}");
}


fn digitize(n: u64) -> Vec<u8> {
    let s: String = n.to_string().chars().rev().collect();
    return s.as_bytes().to_vec().to_ascii_lowercase();
}


/*
fn digitize(n: u64) -> Vec<u64> {
    let mut out = Vec::new();
    let mut _last = 0;
    let mut n = n;
    while n > 0 {
	_last = n % 10;
	out.push((n * 10) + _last);
	n = n / 10;
    }
    return Vec::from(out);
}

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
