
fn main() {
    //let input = [17.0, 16.0, 16.0, 16.0, 16.0, 15.0, 17.0, 17.0, 15.0, 5.0, 17.0, 17.0, 16.0];
    let input = [];
    let out = find_average(&input);
    println!("{}", out);
}

fn find_average(slice: &[f64]) -> f64 {
    let mut out: f64 = 0.0;
    let mut i: f64 = 0.0;
    for x in slice {
        out = out + x;
        i += 1.0 * x/x;
    }
    out  = out / i as f64;
    if i == 0.0 {out = 0.0}
    return out
}

/*
mod tests {
    use crate::find_average;

    #[test]
    fn example() {
        let input = [17.0, 16.0, 16.0, 16.0, 16.0, 15.0, 17.0, 17.0, 15.0, 5.0, 17.0, 17.0, 16.0];
        let a = find_average(&input);
        let b = 200.0 / 13.0;
        assert_eq!(a,b);
    }
}
*/
