enum Message {
    Quit,
    Move { x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

// Enumにメソッドを定義できる
impl Message {
    fn call(&self) {

    }
}
