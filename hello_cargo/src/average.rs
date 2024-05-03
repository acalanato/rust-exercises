fn main() {
    #[test]
    let test: f64 = [17.0, 16.0, 16.0, 16.0, 16.0, 15.0, 17.0, 17.0, 15.0, 5.0, 17.0, 17.0, 16.0];
    println!("{}",find_average(test));
}

fn find_average(slice: &[f64]) -> f64 {
    let mut a: i32 = 0;
    for x in &slice {
        a = a + x
    }
}

