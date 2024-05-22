
fn main() {
    let x = square_digits(9119);
    println!("{x:?}");
}


fn square_digits(num: u64) -> u64 {
    let mut n = num;
    let mut i = 0;
    let mut _out = 0;
    loop {
	n = n / 10;
	_out = n * n * 10 * i;
	i += 1;
	if n == 0 {break}
    }
    return _out;
}
//    num.next_power_of_two()
//    return num;


/*
fn square_digits(num: u64) -> u64 {
    let binding = num.to_string();
    let s = binding.chars().filter_map(|x| x.to_digit(10));
    let mut sq = Vec::new();
    for x in s.into_iter() {
        sq.push(x * x);
    };
    return out;
}
*/
