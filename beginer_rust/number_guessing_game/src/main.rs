/// A function for a number guesing game
use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
	// --snip--
    println!("Guess the number!");
	
	let secret_number = rand::thread_rng().gen_range(1..=100);

	println!("please secret number is: {secret_number}");

	
	loop {
		println!("please input your guess.");
		
		// --snip--
		let mut guess = String::new();

		io::stdin()
			.read_line(&mut guess)
			.expect("Failed to read line");
		
		let guess: u32 = match guess.trim().parse() {
			Ok(num) => num,
			Err(_) => continue,
		};
		
		println!("You guessed: {}", guess);

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
