/// for loop

fn main() {
	let a = [10,20,30, 40, 50];

	for element in a {
		println!("the value is: {element}");
	}
	println!("End");

	for num in (1..4).rev() {
        println!("{num}!");
    }
    println!("LIFTOFF!!!");
}
