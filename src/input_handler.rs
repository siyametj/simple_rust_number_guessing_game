// random_number_guessing/src/input_handler.rs

use std::io::{self, Write};

pub fn take_integer(prompt: &str) -> i32 {
    let mut input = String::new();

    loop {
        print!("{}: ", prompt);
        io::stdout().flush().expect("Failed to flush");

        input.clear();

        // Take input
        match io::stdin().read_line(&mut input) {
            Ok(0) => {
                println!("Program interrupted by user! Goodbye");
                std::process::exit(0);
            }

            Ok(_) => {
                if input.trim().is_empty() {
                    println!("Input can't be empty! Please try again");
                    continue;
                }
            }

            Err(_) => {
                println!("Something went wrong! Goodbye");
                std::process::exit(0);
            }
        }

        // Made String to integer(i32)
        match input.trim().parse::<i32>() {
            Ok(num) => return num,
            Err(_) => {
                println!("Invalid integer! Please enter a valid integer");
                continue;
            }
        }
    }
}

pub fn take_string(prompt: &str) -> String {
    let mut input = String::new();

    loop {
        print!("{}: ", prompt);
        io::stdout().flush().expect("Failed to flush");

        input.clear();

        match io::stdin().read_line(&mut input) {
            Ok(0) => {
                println!("Program interrupted by user! Goodbye");
                std::process::exit(0);
            }

            Ok(_) => {
                if input.trim().is_empty() {
                    println!("Input can't be empty! Please try again");
                    continue;
                } else {
                    return input.trim().to_string();
                }
            }

            Err(_) => {
                println!("Something went wrong! Goodbye");
                std::process::exit(0);
            }
        }
    }
}
