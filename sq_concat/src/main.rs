
fn main() {
    println!("Hello, world!");
}

fn square_digits(num: u64) -> Vec<u32> {
    let mut s = num.to_string().chars().filter_map(|x| x.to_digit(10));
    let mut out = Vec::new();
    for x in s.into_iter() {
        out.push(x * x);
    }
    return out;
}
