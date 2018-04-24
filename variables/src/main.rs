fn five() -> i32 {
    5
}

fn main() {
    let x = 5;

    let y = {
        let x = 3;

        // 式は終端にセミコロンをつけない つけると文になってしまう
        x + 1
    };

    println!("The value of y is: {}, {}", five(), plus_one(6));
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
