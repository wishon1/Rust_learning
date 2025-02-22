// function that acess the element of an array by taking user input
use std::io;

fn access_array() {
	let a = [1, 2, 3, 4, 5];

	println!("Please enter a new index.");
	let mut index = String::new();

	io::stdin()
		.read_line(&mut index)
		.expect("Failed to read line");

	let index: usize = index
		.trim()
		.parse()
		.expect("Index entered was not a number");
	
	let element = a[index];

	println!("The value of the element at {index} is: {element}");
}

fn main() {
	access_array();
}
