use std::fmt;

fn main() {
    println!("Hello, world!");

    another_function(4);
    let x = plus_one(5);
    measure(5, 'h');
    let y = y_exp();
    println!("{y}");
    let test = Test { x: 3.2, y: 4.0, };
    println!("X from struct is: {} and Y is: {}", test.x, test.y);
    println!("The value of the last function is: {x}");
}

fn another_function(x: i32) {
    println!("Goodbye cruel world! {x} times");
}

fn measure (value: i32, unit: char) {
    println!("The measurement is: {value}{unit}");
}

fn y_exp() -> i32 {
    let y = {
	let x = 3;
	x + 1
    };
    return y
}

struct Test {
    x: f64,
    y: f64,
}

fn plus_one(x: i8) -> i8 {
    x + 1
}



impl fmt::Display for Test {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
	write!(f, "x: {}, y: {}", self.x, self.y)
    }
}
