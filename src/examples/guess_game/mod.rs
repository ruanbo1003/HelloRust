
use std::io;
use std::cmp::Ordering;
use rand::Rng;

pub fn tests() {
    println!("Guess Number Game");
    println!("Input your guess.");

    let secret_number: u32 = rand::thread_rng().gen_range(1..=100);

    loop {
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess : u32 = guess.trim().parse().expect("Please input a number");
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small."),
            Ordering::Greater => println!("Too big."),
            Ordering::Equal => {
                println!("Nice.");
                break
            }
        }
        println!("You guessed: {guess}, the secret number is: {secret_number}")
    }

}