/// Calculate rectangle area using structures and methods
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // Method to calculate area
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // Method to check if width is nonzero
    fn width(&self) -> bool {
        self.width > 0
    }

    // Method to check if self can hold another rectangle
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

	// Not a method but an associated function create a square Rectangle 
	fn square(size: u32) -> Self {
		Self {
			width: size,
			height: size,
		}
	}
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };

    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

	let sq = Rectangle::square(5);

    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }

    println!("The area of the rectangle is {} square pixels.", rect1.area());
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
	println!("New sqare created has a value of {:?}", sq)
}
