
use std::io;
fn main() {
    println!("Hello, world!");

    let condtion: bool = true;
    let number = if condition {
        5
    } else {
        "six" // error
    };
    println!("The value of number is: {}", number);

    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .expect("Failed to read line");

    let input: u32 = input.trim().parse()
        .expect("err");
    if input < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }


}
