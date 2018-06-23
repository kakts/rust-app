use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::process;

fn main() {
    println!("Hello, world!");

    // read the command-line arguments.
    // notice: throw panic when invalid unicode string is passed.
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    // Using unwrap_or_else, we can set the original Error handling.
    // if Ok, it returns the value which OK holds.
    // if Err, it calls code in the closure.
    let config = Config::new(&args).unwrap_or_else(|err| {
       println!("Problem parsing arguments: {}", err);
       process::exit(1); 
    });;

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    // open file
    let mut f = File::open(config.filename).expect("file not found");
    let mut contents = String::new();

    // read file 
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    println!("With text:\n {}", contents);
}

// for a readability, create Config type
struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config {query, filename})
    }
}