use std::fs::File;

fn main() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => {
            // ファイルを開く際に問題が会ったのでpanic
            panic!("There was a problem opening the file: {:?}", error);
        }
    };

    let f2 = File::open("dummy.txt").expect("Failed to open ");
}
