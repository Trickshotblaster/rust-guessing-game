use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    'outer: loop {
        println!("--GUESSING GAME--");
        println!("Input your guess:");
        let stop_word = "quit";
        let secret_number = rand::thread_rng().gen_range(1..=100);
        loop {
            let mut guess = String::new();
            
            io::stdin()
                .read_line(&mut guess)
                .expect("Failed to read line");


            println!("You guessed: {guess}");

            if &guess.trim() == &stop_word {
                println!("quit game");
                break 'outer;
            }

            let guess:u32 = match guess.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };

            match guess.cmp(&secret_number) {
                Ordering::Less => println!("Higher"),
                Ordering::Greater => println!("Lower"),
                Ordering::Equal => {
                    println!("Skedoosh!");
                    print!("{esc}[2J{esc}[1;1H", esc = 27 as char); // set to the top of the screen
                    break;
                },
            }
        }
    }
}