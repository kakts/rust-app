extern crate minigrep;

use std::env;
use std::process;

use minigrep::Config;

fn main() {
    println!("Hello, world!");

    // read the command-line arguments.
    // notice: throw panic when invalid unicode string is passed.
    let args: Vec<String> = env::args().collect();

    // Using unwrap_or_else, we can set the original Error handling.
    // if Ok, it returns the value which OK holds.
    // if Err, it calls code in the closure.
    let config = Config::new(&args).unwrap_or_else(|err| {
       process::exit(1); 
    });;

   if let Err(e) = minigrep::run(config) {
       println!("Application error: {}", e);

       process::exit(1);
   }
}
