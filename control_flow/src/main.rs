fn main() {
    //    println!("Hello, world!");
    number(3);
    number(6);
    check_zero(3);
    check_zero(0);
    multiple(6);
    if_let(true);
    ret_mismatch(false);
    ret_loop();
    loop_within();
    w_loop();
    w_loop_b();
    loop_for();
    loop_for_b();
}

fn number (number: u8) {
    if number < 5 {
	println!("Condition was true")
    } else {
	println!("Condition was false")
    }
}

fn check_zero (n:u8) {
    if n != 0 {
	println!("Number isn't zero")
    } else {
	println!("Number is zero")
    }
}

// Multiples conditions

fn multiple(n: u8) {
    let m: &str = "Number is divisible by:";
    if n % 4 == 0{
	println!("{m} 4")
    } else if n % 3 == 0 {
	println!("{m} by 3")
    } else if n % 2 == 0 {
	println!("{m}y 2")
    }
}

fn if_let(b: bool) {
    let number = if b {5} else {6};
    println!("The value inside \"if_let\" is: {number}")
}

fn ret_mismatch (b: bool) {
    let number = if b {"5"} else {"six"};
    println!("The value inside \"ret_mismatch\" is: {number}")

}

fn ret_loop() {
    let mut counter = 0;

    let result = loop {
	counter += 1;
	if counter == 10 {
	    break counter * 2;
	}
    };
    println!("The result of the loop is {result}")

}

fn loop_within() {
    let mut count = 0;
    'counting_up: loop {
	println!("count = {count}");
	let mut remaining = 10;

	loop {
	    println!("remaining = {remaining}");
	    if remaining == 9 {
		break;
	    }
	    if count == 2 {
		break 'counting_up;
	    }
	    remaining -= 1;
	}
	count +=1;
    }
    println!("End count = {count}");
}

fn w_loop() {
    let mut number = 3;
    while number != 0 {
	println!("{number}!");
	number -= 1;
    }
    println!("Boom!!!")
}

fn w_loop_b() {
    for number in (1..4).rev() {
	println!("{number}!");
    }
    println!("Liftoff sounds dumb!");
}

fn loop_for() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
	println!("The value is: {}", a[index]);
	index += 1;
    }
}

fn loop_for_b() {
    let a = [10, 20, 30, 40, 50];
    for element in a {
	println!("The value is: {element}")
    }
}
