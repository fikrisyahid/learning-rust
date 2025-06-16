use std::{cmp::Ordering, io};

use rand::Rng;

pub fn run() {
    println!("Guess the number!");

    let secret_number: i32 = rand::thread_rng().gen_range(-100..=100);
    println!("The secret number address is: {:p}", &secret_number);

    loop {
        println!("Please input your guess: ");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read the line");

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please input a number");
                continue;
            }
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Small!"),
            Ordering::Greater => println!("Too Big!"),
            Ordering::Equal => {
                println!("You Win!");
                break;
            }
        }
    }
}
