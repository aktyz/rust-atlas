use std::io;

use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is {secret_number}");
    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess) // &mut guess - reference,
                               // which gives you a way to let multiple parts of your code
                               // access one piece of data without needing to copy
                               // that data into memory multiple times
                               // just like variables references are immutable by default
        .expect("Failed to read line"); // We put an expectation on the `read_line()` function's `Result``
    println!("You guessed: {guess}")
}
