
fn _series_sum(n: u32) -> String {
    let mut result = 0.0;
    for x in 0..=n {
	result =  result + (1.0 / (1 +(x * 1)) as f32)
    }
    format!("{:.2}", result)
}


fn series_sum(n: u32) -> String {
    (0..=n).into_iter().map(|x|  x as f32 * (1.0 / x as f32))
	.last()
	.unwrap()
	.to_string()
}

fn main() {
    println!("{}", series_sum(4));
}

#[cfg(test)]
mod tests {
    use super::series_sum;
    
    fn test(input: u32, expected: &str) {
        let actual = series_sum(input);
        assert!(actual == expected, "Expected series_sum({input}) to be {expected}, but was {actual}");
    }

    #[test]
    fn sample_tests() {
        test(1, "1.00");
        test(2, "1.25");
        test(3, "1.39");
        test(7, "1.68");
        test(39, "2.26");
        test(0, "0.00");
    }
}
