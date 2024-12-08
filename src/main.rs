use rand::Rng;
use std::{cmp::Ordering, io};
fn main() {
    println!("Welcome to game of Guessing the numbers!");

    println!("Guess and number between 1 to 100: ");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read user input");

        let guess: u32 = match guess.trim().parse() {
            Ok(n) => n,
            Err(_) => {
                println!("Please enter a number");
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Equal => {
                println!("Congratulations! You guessed the number correctly!");
                break;
            }
            Ordering::Less => println!("Too low! Try again."),
            Ordering::Greater => println!("Too high! Try again."),
        }
    }
}
