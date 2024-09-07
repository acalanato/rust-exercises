
fn series_sum(n: u32) -> String {

    //let serie: Vec<_> = (0..n).map(|x| 1.0 * x as f32).collect();
    
    let mut _result = 0.0;
   
    /*
    for x in 0..=n {
	result =  result + (1.0 * (1.0 * (x as f32 * 1.0)) as f32)
    }
    */
    //foo.iter().enumerate().map(|(i, val)| i + *val)

    _result = (0..=n).step_by(2).enumerate()
	.map(|(i, val)| (1.0 /(val as f32 + i as f32)) + i as f32)
	.last().expect("Panic!");

    format!("{:.2}", _result)
}

//    (0..=n.)into_iter().map(|x| x as f32 * (1.0 / x as f32 + (1..)).step_by(3)))

fn _series_sum(n: u32) -> String {
    let serie: Vec<_> = (1..n).step_by(3).collect();
    let mut result = 1.0;
    for (x, _y) in (0..n).into_iter().enumerate() {
	result += 1.0 * serie[x] as f32
    }
    format!("{:.2}",result)
/*
    (0..=n).into_iter().map(|x, n| x as f32 * serie[n])
	.last()
	.unwrap()
	.to_string()
*/
}

fn main() {
    println!("{}", series_sum(2));
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
