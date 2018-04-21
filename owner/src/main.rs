fn main() {
    println!("Hello, world!");

    // Create a String variable.
    // String allocates memory to heap
    let mut s = String::from("Hello");
    s.push_str(", world!");

    println!("{}", s);


    let s1 = String::from("Hello");
    let s2 = s1;
    println!("{}, {}", s1, s2);
}
