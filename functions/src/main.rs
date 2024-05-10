fn main() {
    println!("Hello, world!");

    another_function(4);

    measure(5, 'h');

    let y = y_exp();
}

fn another_function(x: i32) {
    println!("Goodbye cruel world! {x}");
}

fn measure (value: i32, unit: char) {
    println!("The measurement is: {value}{unit}");
}

fn y_exp () {
    let y = {
	let x = 3;
	x + 1;
    };
    println!("The value of y is:{y}");
}
