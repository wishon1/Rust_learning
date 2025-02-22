use std::io;

/// Program that converts Celsius to Fahrenheit and back to Celsius.
fn main() {
    println!("Welcome to the Temperature Converter!");
    println!("Enter the temperature to convert (e.g., 25C or 77F):");

    loop {
        // Create a string to store the user input
        let mut user_input = String::new();

        // Get input from user
        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read line");

        // Remove whitespace
        let input = user_input.trim();

        if input.len() > 2 {
            // Extract the last character (unit: 'C' or 'F')
            let last_character = input.chars().last().unwrap();

            // Extract the numeric part manually
            let mut number_part = String::new();
            for c in input.chars().take(input.len() - 1) {
                number_part.push(c);
            }

            // Convert the number part to a float
            let number: f64 = match number_part.parse() {
                Ok(n) => n,
                Err(_) => {
                    println!("Invalid number format. Please enter a valid number.");
                    continue;
                }
            };

            // Convert temperature based on unit
            match last_character {
                'C' | 'c' => {
                    let fahrenheit = (number * 9.0 / 5.0) + 32.0;
                    println!("{:.2}°C is {:.2}°F", number, fahrenheit);
                }
                'F' | 'f' => {
                    let celsius = (number - 32.0) * 5.0 / 9.0;
                    println!("{:.2}°F is {:.2}°C", number, celsius);
                }
                _ => {
                    println!("Invalid unit. Please use 'C' for Celsius or 'F' for Fahrenheit.");
                    continue;
                }
            }

            // Exit loop after successful conversion
            break;
        } else {
            println!("Invalid input. Please enter a valid temperature (e.g., 30C or 86F).");
		}
	}
}
