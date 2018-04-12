use std::io;

fn main() {
    println!("Guess the number!");
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
}
