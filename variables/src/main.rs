fn main() {
    let x = 5;

    let y = {
        let x = 3;

        // 式は終端にセミコロンをつけない つけると文になってしまう
        x + 1
    };

    println!("The value of y is: {}", y);
}
