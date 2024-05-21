
fn main() {
    let x = square_digits(9119);
    println!("{x:?}");
}

fn square_digits(num: u64) -> u64 {
    let binding = num.to_string();
    let s = binding.chars().filter_map(|x| x.to_digit(10));
    let mut out = Vec::new();
    for x in s.into_iter() {
        out.push(x * x);
    };
    return out;
}
