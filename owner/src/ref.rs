fn main() {
    let s1 = String::from("hello");

    // path reference to function
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);
}

// 所有権を渡すことなく値を参照できる
fn calculate_length(s: &String) -> usize {
    s.len()
}
