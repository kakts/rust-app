use std::env;
fn main() {
    println!("Hello, world!");

    // read the command-line arguments.
    // notice: throw panic when invalid unicode string is passed.
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    // &args[0] is a name of the programme.
    let query = &args[1];
    let filename = &args[2];

    println!("Searching for {}", query);

    println!("In file {}", filename);
}
