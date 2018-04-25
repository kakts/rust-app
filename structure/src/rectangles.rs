struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    // Rectangle構造体のインスタンスを作る
    let rect1 = Rectangle {
        width: 30,
        height: 50
    };

    // rect1のポインタを渡すことでareaに借用する
    // mainは所有権を保ってrect1を使用し続けれる
    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );


    println!("{}", rect1.width);
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
