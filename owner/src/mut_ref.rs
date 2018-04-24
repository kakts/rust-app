fn main() {
    let mut s = String::from("hello");

    change(&mut s);
    println!("{}", s);
}

// this can modify mutable some_string
fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
