
fn main() {

    let x = square_digits(321);
    //    let x = square_digits(123456);
//    let x = isolate(9119);
    println!("{x}");
}

fn square_digits(num: u64) -> u64{
    //    let n = |x: u64| num*x;
    let mut i = 1;
    let mut out = 0;
    while num > 0 {
        out = num - (num % 10).pow(2);
        out -= i;
        i *= 10;
    }
    out
}
/*

fn square_digits(num: u64) -> u64{
    let isolate = |x, y| (x / y) % 10;
    let mut out = num;
    let mut i = 1;
    loop {
        out = out + isolate(num, i).pow(2);
        i += 1;
        if i > 10 {break};
    };
    out
}


fn square_digits(num: u64) -> u64{
    let mut x = 1;
    let mut y = 0;
    let mut z = 0;
    let isolate = |x, y| (x / y) % 10;
    while y < num {
        y = y + isolate(num, x);
        x *= 10;
        z += y * 2;
    }
    z
}


fn isolate(num: u64) {
//    let num: u32 = 1048572;
    let mut i = num;
    
    let mut digits = Vec::new();
    
    while i > 0 {
        let digit = i % 10;
        digits.push(digit);
        i /= 10;
    }
    
    digits.reverse();
    
    for digit in digits {
        println!("{}", digit);
    }
}



fn square_digits(num: u64) -> u64 {
    let mut out = 0;
    for x in 1..num {
        out += num / x
    }
    return out;
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



fn square_digits(num: u64) -> u64 {
    let binding = num.to_string();
    let s = binding.chars().filter_map(|x| x.to_digit(10));
    let mut out = Vec::new();
    for x in s.into_iter() {
        out.push(x * x);
    };
    //    out.into_iter().map(|x| u64::from(x) ).collect();
    //    return from(out: u64);
    //    return u64::from(out)
    //    return out.iter().map(|c| *c ).collect();
    return out;
}
 */

