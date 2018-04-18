fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;

    println!("1: {}, 2: {}, 3: {}", five_hundred, six_point_four, one);

    // array has same type of data.
    let a = [1, 2, 3, 4, 5];

    println!("1: {}, 2: {}", a[0], a[2]);

    another_function();
    add(2, 3);
}

fn another_function() {
    println!("Another function.");
}

fn add(x: u32, y: u32) {
    println!("{}", x + y);
}
