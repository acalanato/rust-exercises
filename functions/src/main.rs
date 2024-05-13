//use std::fmt;

//struct Y_exp(i32);

fn main() {
    println!("Hello, world!");

    another_function(4);

    measure(5, 'h');
    let y = y_exp();
    print!("{y}");
}

fn another_function(x: i32) {
    println!("Goodbye cruel world! {x}");
}

fn measure (value: i32, unit: char) {
    println!("The measurement is: {value}{unit}");
}

fn y_exp() -> i32 {
    let y = {
	let x = 3;
	x + 1
    };
    return y;
}

/*
impl fmt::Display for y_exp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
	write!(f, "{}", self.0)
    }
}
*/
