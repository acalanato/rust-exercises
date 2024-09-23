

fn remove_smallest(numbers: &[u32]) -> Vec<u32> {
    if numbers.len() > 0 {
	let mut min = numbers[0];
	let mut rem = 1;
	let mut out = Vec::new();
	for x in numbers.iter() {
	    if x < &min {
		min.clone_from(x);
	    }
	}
	for x in numbers {
	    if x == &min && rem == 1{
		rem = 0;
		continue;	   
	    } else {
		out.push(*x);
	    }
	}
	return out
    } else {
	return numbers.to_vec()
    }
}

fn _remove_smallest(numbers: &[u32]) -> Vec<u32> {
    let mut n = numbers.to_vec();
    if let Some((pos, _)) = n.iter().enumerate().min_by_key(|&(_, x)| x) {
        n.remove(pos);
    }
    n
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
	dotest(&[0], &[0]);
    }
}
