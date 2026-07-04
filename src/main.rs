// random_number_guessing/src/main.rs

mod helper;
mod input_handler;
mod number_gen;

use helper::clear_screen;
use input_handler::{take_integer, take_string};
use number_gen::generate_secret_number;

fn main() {
    clear_screen();
    println!("Simple Rust Number Guessing generate_secret_number");

    let player_name = take_string("Enter your player name");
    let mut round: i32 = 1;
    loop {
        clear_screen();
        println!(
            "Player: {} | Game Status: Started | Round: {}",
            player_name.to_uppercase(),
            round
        );

        let secret_number = generate_secret_number();

        let mut attempts = 0;

        loop {
            let guess = take_integer("Enter your guess (1-100)");
            attempts += 1;

            if guess > secret_number {
                println!("TOO HIGH! Come down!");
                println!("-----------------------------------------")
            } else if guess < secret_number {
                println!("TOO LOW! Go up!");
                println!("-----------------------------------------")
            } else {
                println!("\nBOOM!!! YOU WIN, {}!", player_name.to_uppercase());
                println!("Total attempts: {}!", attempts);
                break;
            }
        }

        loop {
            let choice = take_string("Do you want to play again? (y/n)");
            if choice.trim().to_lowercase() == "no" || choice.trim().to_lowercase() == "n" {
                print!(
                    "Thanks for playing, {}! See ya later",
                    player_name.to_uppercase()
                );
                std::process::exit(0);
            } else if choice.trim().to_lowercase() == "yes" || choice.trim().to_lowercase() == "y" {
                round += 1;
                break;
            } else {
                println!(
                    "Invalid input! Please input y/yes for run play round: {} else input n/no for exit",
                    round
                );
            }
        }
    }
}
