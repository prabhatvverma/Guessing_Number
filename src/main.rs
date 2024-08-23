use rand::Rng;
use std::cmp::Ordering;
use std::{io, u32};
fn main() {
    println!("Add your guess number!");
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input you guess");
        let mut guess: String = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        // shadowing means we have created a shadow of the variable first one will be neglected
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please Enter a valid number!");
                continue;
            }
        };
        println!("You guessed: {guess}",);
        match guess.cmp(&secret_number) {
            Ordering::Equal => {
                println!("Congratulations You Won The Game!");
                break;
            }
            Ordering::Greater => println!("You Answer is grater than expected number!"),
            Ordering::Less => println!("You Answer is less than expected number!"),
        }
    }
}
