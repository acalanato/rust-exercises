
fn positive_sum(slice: &[i32]) -> i32 {
    let mut out = 0;
    for x in slice {
	if *x >= 0 {
	    out = out + x
	}
    }
    out
}

fn _positive_sum(arr: &[i32]) -> i32 {
    arr.iter().filter(|x| x.is_positive()).sum()
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn some_examples() {
        assert_eq!(positive_sum(&[1,2,3,4,5]), 15);
        assert_eq!(positive_sum(&[1,-2,3,4,5]), 13);
        assert_eq!(positive_sum(&[-1,2,3,4,-5]), 9);
    }
    
    #[test]
    fn empty_list() {
        assert_eq!(positive_sum(&[]), 0);
    }
    
    #[test]
    fn all_negative() {
        assert_eq!(positive_sum(&[-1,-2,-3,-4,-5]), 0);
    }       
}
