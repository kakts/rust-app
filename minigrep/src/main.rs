use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    println!("Hello, world!");

    // read the command-line arguments.
    // notice: throw panic when invalid unicode string is passed.
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    // &args[0] is a name of the programme.
    let config = Config::new(&args);

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
    fn new(args: &[String]) -> Config {
        if args.len() < 3 {
            panic!("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Config {query, filename}
    }
}