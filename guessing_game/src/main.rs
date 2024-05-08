use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    // thread_rng seeds from the os

    println!("The secret number is: {secret_number}");

    loop {
	println!("Please input your guess.");

	let mut guess = String::new();

	io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
	// return a Result that is enum type
	// this enum can be either Ok or Err
	// .expect is a method of Result

	let guess: u32 = match guess.trim().parse() {
	    Ok(num) => num,
	    Err(_) => continue,
	};
	
	println!("You guessed: {guess}");

	match guess.cmp(&secret_number) {
	    Ordering::Less => println!("Too small!"),
	    Ordering::Greater => println!("Too big!"),
	    Ordering::Equal => {
		println!("You win!");
		break;
	    }
	}
    }
}
