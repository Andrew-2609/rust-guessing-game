use io::stdout;
use std::{
    cmp::Ordering,
    io::{self},
};

use rand::Rng;

use ferris_says::say;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);

    styled_print("Guess the number!");

    loop {
        println!("Please, input your guess: ");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                styled_print("You won!");
                break;
            }
        }
    }
}

fn styled_print(message: &str) {
    say(&message, message.len(), &mut stdout()).unwrap();
}
