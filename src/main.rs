use std::cmp::Ordering;
use std::io;

use rand::Rng;

pub struct SecretNumber {
    value: i32,
}

impl SecretNumber {
    pub fn new(value: i32) -> SecretNumber {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {value}.",);
        }

        SecretNumber { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}

fn main() {
    println!("Guess the number!");

    let secret_number: SecretNumber = SecretNumber::new(rand::thread_rng().gen_range(1..=100));

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a number");
                continue;
            }
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number.value) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
