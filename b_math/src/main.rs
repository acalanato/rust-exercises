fn main() {

    print!("
sum   {} should be 11,
minus {} should be -3,
prod  {} should be 25,
div   {} should be 7.

",
            basic_op('+', 4, 7),
            basic_op('-', 15, 18),
            basic_op('*', 5, 5),
            basic_op('/', 49, 7));
}

fn basic_op(operator: char, value1: i32, value2: i32) -> i32 {
    match operator {
        '+' => value1 + value2,
        '-' => value1 - value2,
        '*' => value1 * value2,
        '/' => value1 / value2,
        _=> panic!(),
    }
}
