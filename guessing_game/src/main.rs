use std::cmp::Ordering;
use std::io;

use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number: u32 = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is {secret_number}");
    loop {
        println!("Please input your guess.");

        let mut guess: String = String::new();

        io::stdin()
            .read_line(&mut guess) // &mut guess - reference,
                                // which gives you a way to let multiple parts of your code
                                // access one piece of data without needing to copy
                                // that data into memory multiple times
                                // just like variables references are immutable by default
            .expect("Failed to read line"); // We put an expectation on the `read_line()` function's `Result``
        let guess: u32 = guess.trim().parse()
            .expect("Please type a number!");
        println!("You guessed: {guess}");
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
