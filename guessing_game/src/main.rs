// lets Rust know we'll be using that external dep
extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;
fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess.");
        // let: create variables
        // mut: let the variable mutable
        // String::new() is a static method of String.
        let mut guess = String::new();
        // read_line returns the value of Result type.
        // Result type has expect() method.
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");
        println!("You guessed: {}", guess);

        // convert to the unsigned int
        // trim() method eliminates \n from the input
        // parse method on strings parses a string into some kind of number.
        // the colon after guess tells Rust we'll annotate the variable's type
        let guess: u32 = guess.trim().parse()
            .expect("Please type a number!");

        // Ordering is another enum, like Result.
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
