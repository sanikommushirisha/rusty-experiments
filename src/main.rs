mod guess_input;
mod guessing_game;
use std::cmp::Ordering::{Less, Greater, Equal};

fn main() {
    println!("Guess the number!");
    let secret_number: u32 = rand::random_range(1..=100);

    println!("The secret number is: {secret_number}");

    println!("Please input your guess.");

    loop {    
        let mut guess = String::new();
        std::io::stdin().read_line(&mut guess).expect("Failed to read line");
        println!("You guessed: {guess}");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number!");
                continue;
            },
        };
        match guess.cmp(&secret_number) {
            Less => println!("Too small!"),
            Greater => println!("Too big!"),
            Equal => {
                println!("You win!");
                break;
            },
        }
    }

}