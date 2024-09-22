fn remove_smallest(numbers: &[u32]) -> Vec<u32> {
    let mut rem: u32 = 0;
    let out = numbers;
    for x in out.iter() {
	if x > &rem {
	    rem.clone_from(x);
	}
    }
    println!("{:?}", rem);
    out.to_vec()
}


fn main() {

    println!("{:?}", remove_smallest(&[1,2,3,4,5]));
    
    println!("Hello, world!");
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::remove_smallest;
    
    const ERR_MSG: &str = "\nYour result (left) did not match the expected output (right)";
    
    fn dotest(a: &[u32], expected: &[u32]) {
        assert_eq!(remove_smallest(a), expected, "{ERR_MSG} with numbers = {a:?}")
    }

    #[test]
    fn fixed_tests() {
        dotest(&[1, 2, 3, 4, 5], &[2, 3, 4, 5]);
        dotest(&[1, 2, 3, 4], &[2, 3, 4]);
        dotest(&[5, 3, 2, 1, 4], &[5, 3, 2, 4]);
        dotest(&[1, 2, 3, 1, 1], &[2, 3, 1, 1]);
    }
}
