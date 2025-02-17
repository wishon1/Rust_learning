/// function to explain the use of type annotation with multiple characters

fn main() {
	print_labeled_measurement(5, 'h');
}

fn print_labeled_measurement(value: i32, unit_label: char) {
	println!("The measurement is: {value}{unit_label}");
}
