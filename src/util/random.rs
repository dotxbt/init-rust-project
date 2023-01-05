use std::{cmp::Ordering, io};

use rand::Rng;

pub fn guest_number() {
    println!("Generate");
    let ini_random = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is: {ini_random}");

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        println!("You guessed: {guess}");

        let guess: u32 = guess.trim().parse().expect("Please type a number!");

        match guess.cmp(&ini_random) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
