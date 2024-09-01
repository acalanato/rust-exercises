fn  nb_year(p0: i32, percent: f64, aug: i32, p: i32)-> i32 {
    let mut n_years = 0;
    let mut population = p0.clone();
    while population <= p {
	population = population + ((population as f64 * (percent / 100.0)) as i32) + aug;
	n_years += 1;
    }
    n_years
}

fn  _nb_year(p0: i32, percent: f64, aug: i32, p: i32)-> i32 {
    let (mut population, percent, imigrating, target) = (p0 as f64, percent, aug as f64, p as f64);
    let mut result = 0;
    while population <= target {
	population = population + (population * (percent / 100.0) + imigrating);
	if population <= target {result += 1}
    }
    result
}

fn main() {
    println!("{}", nb_year(1500, 5.0, 100, 5000));
    println!("{}", nb_year(1500000, 2.5, 10000, 2000000));
    println!("{}", nb_year(1500000, 0.0, 10000, 2000000));

}

#[cfg(test)]
    mod tests {
    use super::*;

    fn dotest(p0: i32, percent: f64, aug: i32, p: i32, exp: i32) -> () {
        println!("p0: {:?};", p0);
        println!("percent: {:?};", percent);
        println!("aug: {:?};", aug);
        println!("p: {:?};", p);
        let ans =nb_year(p0, percent, aug, p);
        println!("actual:\n{:?};", ans);
        println!("expect:\n{:?};", exp);
        println!("{};", ans == exp);
        assert_eq!(ans, exp);
        println!("{};", "-");
    }

    #[test]
    fn basic_tests() {
        dotest(1500, 5.0, 100, 5000, 15);
        dotest(1500000, 2.5, 10000, 2000000, 10);
	dotest(1500000, 0.0, 10000, 2000000, 50)
        
    }
}
