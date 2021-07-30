use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    println!("Guess the number!");
    let secretnumber = rand::thread_rng().gen_range(1..101); // generating a random number
	// println!("The secret number is {}", secretnumber);
    loop { // allowing multiple guesses w/ looping
        println!("Please input your guess: ");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line."); // handling potential errors
        let guess: u8 = match guess.trim().parse() {
            // use match to handle an error
            Ok(number) => number,
            Err(_) => {
                println!("Input a number from 1 to 100!");
                continue;
            }
        };
        println!("You guessed: {}", guess);
        match guess.cmp(&secretnumber) {
            // comparing a guess to a secret number
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break; // quitting after a correct guess
            }
        }
    }
}
