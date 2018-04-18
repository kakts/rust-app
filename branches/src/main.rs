
use std::io;
fn main() {
    println!("Hello, world!");

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
