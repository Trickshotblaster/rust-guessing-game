use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("--GUESSING GAME--");
    println!("Input your guess:");
    loop {
        let secret_number = rand::thread_rng().gen_range(1..=100);
        loop {
            let mut guess = String::new();
            
            io::stdin()
                .read_line(&mut guess)
                .expect("Failed to read line");

            println!("You guessed: {guess}");
            
            let guess:i32 = guess.trim().parse().expect("Please type a valid number");

            match guess.cmp(&secret_number) {
                Ordering::Less => println!("Higher"),
                Ordering::Greater => println!("Lower"),
                Ordering::Equal => println!("Skedoosh!"),
            }
        }
    }
}