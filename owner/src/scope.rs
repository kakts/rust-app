fn main() {

    // it moves return value to s1
    let s1 = gives_ownership();

    // s2 goes into scope
    let s2 = String::from("hello");

    // s2 is moved to this _function
    // return value moves to s3
    let s3 = takes_and_gives_back(s2);
}
// s3 goes out from scope, it's dropped.


fn gives_ownership() -> String {
    let some_string = String::from("Hello");

    // some_string is returned to caller function.
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}
